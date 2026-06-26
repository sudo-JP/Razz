//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{ast::expression::UnOpKind, ir::{Temp, basic_block::BlockId, hir::{hir::HIRFunctionParam, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRFunction, HIRProgram, HIRStmt}}, ssa::ssa::{PhiArg, SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram, SSATerminator}}
};


pub struct HIRStructurizer {
    program: HIRProgram,
}

#[derive(Debug)]
enum DFSResult {
    ForwardEdge(Vec<HIRStmt>),
    BackEdge {
        pointing_to_id: BlockId,
        instrs: Vec<HIRStmt>
    },
}

const BLOCK_MAP_ERR: &str = "all blocks must live in block map";

impl HIRStructurizer {
    pub fn new() -> Self {
        Self {
            program: HIRProgram { functions: vec![] },
        }
    }

    pub fn structurize(mut self, ssa_prog: SSAProgram) -> HIRProgram {
        for function in ssa_prog.functions {
            self.structurize_fn(function);
        }
        self.program
    }

    fn structurize_fn(&mut self, function: SSAFunction) {
        if function.blocks.len() == 0 {
            return;
        }

        // Construct map for DFS
        let mut block_map: HashMap<BlockId, &SSABlock> = HashMap::new();
        for block in &function.blocks {
            block_map.insert(block.id, block);
        }

        // Perform DFS 
        let mut visited: HashSet<BlockId> = HashSet::new();
        let mut visiting: HashSet<BlockId> = HashSet::new();
        let dfs_res = self.dfs(
            &function.block_id, 
            &block_map, 
            &mut visited, 
            &mut visiting,
            None
        );

        let DFSResult::ForwardEdge(block) = dfs_res else {
            unreachable!("function block must be a forward edge")
        };

        let params =  function.params.iter()
            .map(|p| HIRFunctionParam {
                name: p.name.to_string(),
                ty: p.ty
            })
            .collect();

        let function_stmt = HIRFunction {
            name: function.name.to_string(),
            params,
            block,
            return_ty: function.return_ty,
        };
        self.program.functions.push(function_stmt);
    }

    fn process_loop_edge(
        &mut self,
        node_id: BlockId,
        pointing_to_id: BlockId,
        mut instrs: Vec<HIRStmt>,
        mut after: Vec<HIRStmt>,
        mut curr_instrs: Vec<HIRStmt>,
        cond: HIRExpr,
        curr_phis: &[(&Temp, &[PhiArg])],
        block_map: &HashMap<BlockId, &SSABlock>,
        true_label: BlockId
    ) -> DFSResult {
        if pointing_to_id == node_id {
            // This means we in a loop
            let mut decl: Vec<HIRStmt> = vec![];
            let mut updates: Vec<HIRStmt> = vec![];

            for (target, args) in curr_phis {
                for arg in *args {
                    let expr = self.structurize_operand(&arg.operand);
                    let target = **target;
                    if !self.is_reachable(block_map, true_label, arg.from_id) {
                        decl.push(HIRStmt::Assign { target, expr });
                    } else {
                        updates.push(HIRStmt::Assign { target, expr });
                    }
                }
            }

            instrs.append(&mut updates);
            let for_stmt = HIRStmt::While {
                cond,
                block: instrs,
            };
            curr_instrs.append(&mut decl);
            curr_instrs.push(for_stmt);
            curr_instrs.append(&mut after);
            DFSResult::ForwardEdge(curr_instrs)
        } else {
            // Otherwise bubble up back edge if cycle not detected
            curr_instrs.append(&mut instrs);
            curr_instrs.append(&mut after);
            DFSResult::BackEdge {
                pointing_to_id,
                instrs: curr_instrs,
            }
        }
    }

    /// DFS to detect cycle to check if it's a regular 
    /// flow statement, or loop 
    fn dfs(&mut self, 
        node_id: &BlockId, 
        block_map: &HashMap<BlockId, &SSABlock>, 
        visited: &mut HashSet<BlockId>,
        visiting: &mut HashSet<BlockId>,
        stop_at: Option<BlockId>) 
    -> DFSResult {
        if let Some(id) = stop_at 
        && id == *node_id {
            return DFSResult::ForwardEdge(vec![]);
        } else if visited.get(node_id).is_some() {
            // If already in visited, it is finished, skip
            return DFSResult::ForwardEdge(vec![])
        } else if visiting.get(node_id).is_some() {
            // Otherwise, back edge detected
            return DFSResult::BackEdge { 
                pointing_to_id: *node_id, 
                instrs: vec![] 
            };
        }

        // Mark current node as visiting
        visiting.insert(*node_id);

        // Get node neighbour
        let node = block_map.get(node_id).unwrap();

        let mut curr_phis: Vec<(&Temp, &[PhiArg])> = Vec::new();
        let mut curr_instrs = Vec::with_capacity(node.instrs.len());

        for instr in &node.instrs {
            if let SSAInstruction::Phi { target, args } = instr {
                curr_phis.push((target, args));
            } else {
                curr_instrs.push(self.structurize_instr(instr));
            }
        }

        let res = match &node.term {
            // Base case
            SSATerminator::Return(opr) => {
                let opr_expr = self.structurize_operand(opr);
                let ret_stmt = HIRStmt::Return(opr_expr);
                curr_instrs.push(ret_stmt);
                DFSResult::ForwardEdge(curr_instrs)
            },

            // Recursive case
            SSATerminator::Goto(id) => {
                let neigh_res = self.dfs(id, block_map, visited, visiting, stop_at);
                match neigh_res {
                    DFSResult::ForwardEdge(mut stmts) => {
                        curr_instrs.append(&mut stmts);
                        DFSResult::ForwardEdge(curr_instrs)
                    }, 
                    DFSResult::BackEdge { pointing_to_id, mut instrs } => {
                        curr_instrs.append(&mut instrs);
                        DFSResult::BackEdge {
                            instrs: curr_instrs, 
                            pointing_to_id,
                        }
                    }
                }
            }
            SSATerminator::IfGoto { cond, true_label, false_label } => {
                // Inlining conditions
                let cond = if let SSAOperand::Temp(t) = cond {
                    let option_idx = curr_instrs.iter()
                        .position(|stmt| 
                        if let HIRStmt::Assign { target, .. } = stmt {
                                *t == *target
                            } else { false }
                        );

                    match option_idx {
                        Some(idx) => {
                            let HIRStmt::Assign { expr, .. } = curr_instrs.remove(idx) else {
                                unreachable!("Must resolved as above")
                            };
                            expr
                        },
                        None => self.structurize_operand(cond)
                    }
                } else {
                    self.structurize_operand(cond)
                };

                // BFS to find convergence path
                let mut queue: VecDeque<(BlockId, bool)> = VecDeque::new();
                queue.push_back((*true_label, true));
                queue.push_back((*false_label, false));
                let convergence_path = self.find_convergence_path(
                    &mut queue, 
                    block_map, 
                    *node_id
                );

                let mut res = |label| self.dfs(
                    label, 
                    block_map, 
                    visited, 
                    visiting, 
                    convergence_path
                );

                let true_res = res(true_label);
                let false_res = res(false_label);

                let cond_phi = cond.clone();

                let combined = match (true_res, false_res) {
                    (DFSResult::ForwardEdge(then), 
                    DFSResult::ForwardEdge(else_)) => {
                        // Regular if 
                        let if_stmt = HIRStmt::If { 
                            cond, 
                            body: then, 
                            else_body: else_,
                        };
                        curr_instrs.push(if_stmt);
                        DFSResult::ForwardEdge(curr_instrs)
                    }, 
                    (DFSResult::BackEdge { pointing_to_id, instrs },
                    DFSResult::ForwardEdge(after)) => {
                        // Loop is true body
                        self.process_loop_edge(
                            *node_id, 
                            pointing_to_id, 
                            instrs, 
                            after, 
                            curr_instrs, 
                            cond, 
                            &curr_phis, 
                            block_map,
                            *true_label
                        )
                    },
                    (DFSResult::ForwardEdge(after), 
                    DFSResult::BackEdge { pointing_to_id, instrs }) => {
                        // This code shouldn't occurs, but will handle it anyway 
                        let cond = HIRExpr::UnOp { 
                            op: UnOpKind::Not, 
                            value: Box::new(cond),
                        };
                        self.process_loop_edge(
                            *node_id, 
                            pointing_to_id, 
                            instrs, 
                            after, 
                            curr_instrs, 
                            cond, 
                            &curr_phis, 
                            block_map,
                            *false_label
                        )
                    }, 
                    _ => unreachable!("should be taken care by BFS")
                };

                if let Some(conv_label) = convergence_path {
                    let conv_node = block_map.get(&conv_label)
                        .expect(BLOCK_MAP_ERR);

                    let phis = conv_node.instrs.iter()
                        .filter_map(|instr| match instr {
                            SSAInstruction::Phi { target, args } => 
                                Some((target, args)),
                                _ => None,
                        })
                        .collect::<Vec<_>>();

                    let phi_assigns = phis.iter()
                        .map(|(target, args)| {
                            let then_val = self.resolve_phi_value(*true_label, args, block_map);
                            let else_val = self.resolve_phi_value(*false_label, args, block_map);

                            let if_expr = HIRExpr::If { 
                                cond: Box::new(cond_phi.clone()), 
                                then: Box::new(then_val), 
                                else_: Box::new(else_val),
                            };

                            HIRStmt::Assign { 
                                target: **target, 
                                expr: if_expr 
                            }
                        })
                        .collect::<Vec<_>>();
                    let conv_res = self.dfs(&conv_label, block_map, visited, visiting, stop_at);
                    let combined = self.append_stmt_to_result(combined, phi_assigns);
                    self.merge_results(combined, conv_res)
                } else {
                    combined
                }

            },
        };
        // Finished visiting
        visiting.remove(node_id);

        // Mark current node as visited
        visited.insert(*node_id);

        res
    }

    fn append_stmt_to_result(&self, res: DFSResult, mut stmts: Vec<HIRStmt>) -> DFSResult {
        match res {
            DFSResult::ForwardEdge(mut fwd_stmts) => {
                fwd_stmts.append(&mut stmts);
                DFSResult::ForwardEdge(fwd_stmts)
            }
            DFSResult::BackEdge { pointing_to_id, mut instrs } => {
                instrs.append(&mut stmts);
                DFSResult::BackEdge { pointing_to_id, instrs }
            }
        }
    }

    /// First contains orignal result
    /// Second contains the convergence path result
    fn merge_results(&self, first: DFSResult, second: DFSResult) -> DFSResult {
        match (first, second) {
            (DFSResult::ForwardEdge(mut first_fwd),
            DFSResult::ForwardEdge(mut second_fwd)) => {
                first_fwd.append(&mut second_fwd);
                DFSResult::ForwardEdge(first_fwd)
            },
            (DFSResult::ForwardEdge(mut first_fwd),
            DFSResult::BackEdge { pointing_to_id, mut instrs }) => {
                first_fwd.append(&mut instrs);
                DFSResult::BackEdge { pointing_to_id, instrs: first_fwd }
            },
            (DFSResult::BackEdge { pointing_to_id, mut instrs },
            DFSResult::ForwardEdge(mut second_fwd)) => {
                instrs.append(&mut second_fwd);
                DFSResult::BackEdge { pointing_to_id, instrs }
            }
            (DFSResult::BackEdge { pointing_to_id: first_id, instrs: mut first_instrs },
            DFSResult::BackEdge { pointing_to_id: second_id, instrs: mut second_instrs }) => {
                // Should be same value, since bubbling from the same loop
                assert_eq!(first_id, second_id);
                first_instrs.append(&mut second_instrs);
                DFSResult::BackEdge { pointing_to_id: first_id, instrs: first_instrs }
            }
        }
    }

    /// This algorithm is BFS 
    /// Precondition: Takes in a queue populated with nodes 
    /// it want to visit, the boolean ancestor of the node,
    /// and its neighbour map
    /// Return: convergence path if found
    fn find_convergence_path(&self,
        queue: &mut VecDeque<(BlockId, bool)>,
        block_map: &HashMap<BlockId, &SSABlock>,
        parent: BlockId
    ) -> Option<BlockId> {

        // Boolean is used to find mismatching flag 
        // since say a true label have its own divergence 
        // and convergence path, but not the join path of 
        // the false label, we could incorrectly compute 
        // the join path.
        // Thus we have to find visited pair, such that 
        // one visiting node flag is true, visited is false
        // and vice versa. 
        let mut visited: HashMap<BlockId, bool> = HashMap::new();

        while let Some((node_id, curr_flag)) = queue.pop_front() {
            if let Some(ancestor) = visited.get(&node_id) {
                // When the true and false differs
                if *ancestor != curr_flag {
                    return Some(node_id);
                } 
                // Otherwise already visited
                else {
                    continue;
                }
            } else if parent == node_id {
                continue;
            }

            // Get neighbour
            // Precondition that block map populates with node ids
            let node = block_map.get(&node_id).unwrap();
            match &node.term {
                SSATerminator::Return(_) => {}, 
                SSATerminator::Goto(neigh_id) => {
                    queue.push_back((*neigh_id, curr_flag));
                }, 
                SSATerminator::IfGoto { true_label, false_label, .. } =>
                {
                    queue.push_back((*true_label, curr_flag));
                    queue.push_back((*false_label, curr_flag));
                }
            }

            visited.insert(node_id, curr_flag);
        }
        None
    }

    /// Check if label is contains in phi args, if it is,
    /// check for the value that arg holds, to resolve it. 
    /// If a value right there, aka the phi's from points 
    /// to the current block label, then just take the value. 
    /// Otherwise, if a block yields more decision, 
    /// recurse and construct a nested if statement, 
    /// containing the in question divergence path
    fn resolve_phi_value(&self, 
        label: BlockId, 
        phi_args: &[PhiArg], 
        block_map: &HashMap<BlockId, &SSABlock>
    ) -> HIRExpr {
        // Base case 
        if let Some(arg) = phi_args.iter()
            .find(|arg| arg.from_id == label) {
            return self.structurize_operand(&arg.operand);
        }

        // If does not exist in the phi, recurse
        let block = block_map.get(&label)
            .expect(BLOCK_MAP_ERR);
        match &block.term {
            SSATerminator::Return(_) => unreachable!("phi resolution should never walk into a return"),
            SSATerminator::Goto(goto_label) => {
                self.resolve_phi_value(*goto_label, phi_args, block_map)
            },
            SSATerminator::IfGoto { cond, true_label, false_label } => {

                let if_expr = self.resolve_phi_value(*true_label, phi_args, block_map);
                let else_expr = self.resolve_phi_value(*false_label, phi_args, block_map);

                HIRExpr::If { 
                    cond: Box::new(self.structurize_operand(cond)), 
                    then: Box::new(if_expr), 
                    else_: Box::new(else_expr), 
                }
            },
        }
    }

    /// DFS to check if given from's node, 
    /// can we reach to's node
    fn is_reachable(&self, 
        block_map: &HashMap<BlockId, &SSABlock>, 
        from_id: BlockId,
        to_id: BlockId
    ) -> bool {
        let mut stack = vec![from_id];
        let mut visited: HashSet<BlockId> = HashSet::new();

        while let Some(curr_id) = stack.pop() {
            if curr_id == to_id {
                return true;
            } else if visited.get(&curr_id).is_some() {
                continue;
            }

            let curr = block_map.get(&curr_id)
                .expect(BLOCK_MAP_ERR);

            match curr.term {
                SSATerminator::Return(_) => {},
                SSATerminator::Goto(label) => stack.push(label),
                SSATerminator::IfGoto { true_label, false_label, .. }
                    => { 
                    stack.push(false_label);
                    stack.push(true_label);
                }
            }


            visited.insert(curr_id);
        }
        false
    }

    fn structurize_operand(&self, operand: &SSAOperand) -> HIRExpr {
        match operand {
            SSAOperand::Temp(t) => HIRExpr::Temp(*t),
            SSAOperand::Const(c) => HIRExpr::Const(c.clone()),
        }
    }

    fn structurize_instr(&mut self, ssa_instr: &SSAInstruction) -> HIRStmt {
        match ssa_instr {
            SSAInstruction::BinOp { target, lhs, op, rhs } => {
                let lhs = Box::new(self.structurize_operand(lhs));
                let rhs = Box::new(self.structurize_operand(rhs));
                let binop = HIRExpr::BinOp{ 
                    lhs,
                    op: *op,
                    rhs, 
                };
                HIRStmt::Assign { target: *target, expr: binop }
            }, 
            SSAInstruction::UnOp { target, op, value } => {
                let value = Box::new(self.structurize_operand(value));
                let unop = HIRExpr::UnOp{ 
                    op: *op, 
                    value 
                };
                HIRStmt::Assign { target: *target, expr: unop }
            },
            SSAInstruction::Call { target, args, func } => {
                let fn_call = HIRExpr::FunctionCall{ 
                    name: func.to_string(), 
                    args: args.iter()
                        .map(|arg| self.structurize_operand(arg))
                        .collect(),
                };
                if let Some(t) = target {
                    HIRStmt::Assign { target: *t, expr: fn_call }
                } else {
                    HIRStmt::Expr(fn_call)
                }
            },
            SSAInstruction::FieldLoad { target, obj, key } => {
                let obj = Box::new(self.structurize_operand(obj));
                let struct_access = HIRExpr::FieldAccess{ 
                    obj: obj, 
                    key: key.to_string() 
                };
                HIRStmt::Assign { target: *target, expr: struct_access }
            },
            SSAInstruction::FieldStore { obj, key, value } => {
                let obj = self.structurize_operand(obj);
                let value = self.structurize_operand(value);
                HIRStmt::FieldStore{ 
                    obj, 
                    key: key.to_string(), 
                    value, 
                }
            },
            SSAInstruction::Copy { target, value } => {
                let expr = self.structurize_operand(value);
                HIRStmt::Assign { target: *target, expr }
            }, 
            SSAInstruction::Construct { target, ty, fields } => {
                let struct_lit = HIRExpr::StructLiteral{ 
                    ty: *ty, 
                    fields: fields.iter()
                        .map(|ssa_field| HIRFieldInit {
                            name: ssa_field.name.to_string(),
                            value: self.structurize_operand(&ssa_field.value),
                        })
                        .collect()
                };

                HIRStmt::Assign { target: *target, expr: struct_lit }
            },
            SSAInstruction::HTTPGet { target, ep } => {
                let expr = HIRExpr::HTTPRequest(*ep);
                HIRStmt::Assign { target: *target, expr }
            },
            SSAInstruction::HTTPWrite { method, ep, value } => {
                let body = self.structurize_operand(value);

                HIRStmt::HTTPRequest { method: *method, ep: *ep, body }
            },
            SSAInstruction::Phi { .. } => unreachable!("Phi must be dealt with separately"),
        }
    }
}
