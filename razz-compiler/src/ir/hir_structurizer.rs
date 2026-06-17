//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{ast::expression::UnOpKind, ir::{basic_block::BlockId, hir::HIRFunctionParam, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRFunction, HIRProgram, HIRStmt}, ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram, SSATerminator}
}};


pub struct HIRStructurizer {
    program: HIRProgram,
}

enum DFSResult {
    ForwardEdge(Vec<HIRStmt>),
    BackEdge {
        pointing_to_id: BlockId,
        instrs: Vec<HIRStmt>
    },
}

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

    fn dfs(&mut self, 
        node_id: &BlockId, 
        block_map: &HashMap<BlockId, &SSABlock>, 
        visited: &mut HashSet<BlockId>,
        visiting: &mut HashSet<BlockId>,
        stop_at: Option<BlockId>) 
    -> DFSResult {
        if visited.get(node_id).is_some() {
            // If already in visited, it is finished, skip
            return DFSResult::ForwardEdge(vec![])
        } else if let Some(id) = stop_at 
        && id == *node_id {
            return DFSResult::ForwardEdge(vec![]);
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

        let mut curr_instrs = node.instrs.iter()
            .filter_map(|instr| self.structurize_instr(instr))
            .collect::<Vec<HIRStmt>>();

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
                let neigh_res = self.dfs(id, block_map, visited, visiting, None);
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
                let true_res = self.dfs(true_label, block_map, visited, visiting, None);
                let false_res = self.dfs(false_label, block_map, visited, visiting, None);
                let cond = self.structurize_operand(cond);

                match (true_res, false_res) {
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
                    (DFSResult::BackEdge { pointing_to_id, mut instrs },
                    DFSResult::ForwardEdge(mut after)) => {
                        if pointing_to_id == *node_id {
                            // Loop is true body
                            let for_stmt = HIRStmt::While { 
                                cond, 
                                block: instrs,
                            };
                            curr_instrs.push(for_stmt);
                            curr_instrs.append(&mut after);
                            DFSResult::ForwardEdge(curr_instrs)
                        } else {
                            curr_instrs.append(&mut instrs);
                            curr_instrs.append(&mut after);
                            DFSResult::BackEdge { 
                                pointing_to_id, 
                                instrs: curr_instrs 
                            }
                        }
                    },
                    (DFSResult::ForwardEdge(mut after), 
                    DFSResult::BackEdge { pointing_to_id, mut instrs }) => {
                        // This code shouldn't occurs, but will handle it anyway 
                        let cond = HIRExpr::UnOp { 
                            op: UnOpKind::Not, 
                            value: Box::new(cond),
                        };
                        if pointing_to_id == *node_id {
                            // Loop is false body
                            let for_stmt = HIRStmt::While { 
                                cond, 
                                block: instrs
                            };
                            curr_instrs.push(for_stmt);
                            curr_instrs.append(&mut after);
                            DFSResult::ForwardEdge(curr_instrs)
                        } else {
                            curr_instrs.append(&mut instrs);
                            curr_instrs.append(&mut after);
                            DFSResult::BackEdge { 
                                pointing_to_id, 
                                instrs: curr_instrs 
                            }
                        }
                    }, 
                    _ => todo!()
                }

            },
        };
        // Finished visiting
        visiting.remove(node_id);

        // Mark current node as visited
        visited.insert(*node_id);

        res
    }

    /// This algorithm is BFS 
    /// Precondition: Takes in a queue populated with nodes its want to visit
    /// and its neighbour map
    /// Return: convergence path if found
    fn find_convergence_path(&self,
        queue: &mut VecDeque<BlockId>,
        block_map: &HashMap<BlockId, &SSABlock>
    ) -> Option<BlockId> {

        let mut visited: HashSet<BlockId> = HashSet::new();
        while let Some(node_id) = queue.pop_front() {
            // Get neighbour
            // Precondition that block map populates with node ids
            let node = block_map.get(&node_id).unwrap();
            match &node.term {
                SSATerminator::Return(_) => {}, 
                SSATerminator::Goto(neigh_id) => {
                    if let Some(_) = visited.get(neigh_id) {
                        return Some(*neigh_id);
                    }
                    queue.push_back(*neigh_id);
                }, 
                SSATerminator::IfGoto { cond, true_label, false_label } =>
                {

                }
            }

            visited.insert(node_id);
        }
        None
    }

    fn structurize_operand(&self, operand: &SSAOperand) -> HIRExpr {
        match operand {
            SSAOperand::Temp(t) => HIRExpr::Temp(*t),
            SSAOperand::Const(c) => HIRExpr::Const(c.clone()),
        }
    }

    fn structurize_instr(&mut self, ssa_instr: &SSAInstruction) -> Option<HIRStmt> {
        match ssa_instr {
            SSAInstruction::BinOp { target, lhs, op, rhs } => {
                let lhs = Box::new(self.structurize_operand(lhs));
                let rhs = Box::new(self.structurize_operand(rhs));
                let binop = HIRExpr::BinOp{ 
                    lhs,
                    op: *op,
                    rhs, 
                };
                Some(HIRStmt::Assign { target: *target, expr: binop })
            }, 
            SSAInstruction::UnOp { target, op, value } => {
                let value = Box::new(self.structurize_operand(value));
                let unop = HIRExpr::UnOp{ 
                    op: *op, 
                    value 
                };
                Some(HIRStmt::Assign { target: *target, expr: unop })
            },
            SSAInstruction::Call { target, args, func } => {
                let fn_call = HIRExpr::FunctionCall{ 
                    name: func.to_string(), 
                    args: args.iter()
                        .map(|arg| self.structurize_operand(arg))
                        .collect(),
                };
                if let Some(t) = target {
                    Some(HIRStmt::Assign { target: *t, expr: fn_call })
                } else {
                    Some(HIRStmt::Expr(fn_call))
                }
            },
            SSAInstruction::FieldLoad { target, obj, key } => {
                let obj = Box::new(self.structurize_operand(obj));
                let struct_access = HIRExpr::FieldAccess{ 
                    obj: obj, 
                    key: key.to_string() 
                };
                Some(HIRStmt::Assign { target: *target, expr: struct_access })
            },
            SSAInstruction::FieldStore { obj, key, value } => {
                let obj = self.structurize_operand(obj);
                let value = self.structurize_operand(value);
                Some(HIRStmt::FieldStore{ 
                    obj, 
                    key: key.to_string(), 
                    value, 
                })
            },
            SSAInstruction::Copy { target, value } => {
                let expr = self.structurize_operand(value);
                Some(HIRStmt::Assign { target: *target, expr })
            }, 
            SSAInstruction::Construct { target, ty, fields } => {
                let struct_lit = HIRExpr::StructLiteral{ 
                    ty: *ty, 
                    fields: fields.iter()
                        .map(|ssa_field| HIRFieldInit{
                            name: ssa_field.name.to_string(),
                            value: self.structurize_operand(&ssa_field.value),
                        })
                        .collect()
                };

                Some(HIRStmt::Assign { target: *target, expr: struct_lit })
            },
            SSAInstruction::HTTPGet { target, ep } => {
                let expr = HIRExpr::HTTPRequest(*ep);
                Some(HIRStmt::Assign { target: *target, expr })
            },
            SSAInstruction::HTTPWrite { method, ep, value } => {
                let body = self.structurize_operand(value);

                Some(HIRStmt::HTTPRequest { method: *method, ep: *ep, body })
            },
            SSAInstruction::Phi { .. } => None,
        }
    }
}
