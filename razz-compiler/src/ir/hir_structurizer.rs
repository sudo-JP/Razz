//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use std::{collections::{HashMap, HashSet}, mem};

use crate::ir::{basic_block::BlockId, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRProgram, HIRStmt}, ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram, SSATerminator}
};


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
        
        let mut stack: Vec<BlockId> = Vec::with_capacity(function.blocks.len());
        let mut visited: HashSet<BlockId> = HashSet::new();
        stack.push(function.blocks[0].id);

        while !stack.is_empty() {
            let node_id = stack.pop().unwrap();
            let node = block_map.get(&node_id).unwrap();

            visited.insert(node_id);
            match &node.term {
                SSATerminator::Return(opr) => {},
                SSATerminator::Goto(id) => stack.push(*id),
                SSATerminator::IfGoto { true_label, false_label, .. } => {
                    stack.push(*true_label);
                    stack.push(*false_label);
                }
            }
        }
    }

    fn dfs(&mut self, 
        node_id: &BlockId, 
        block_map: &HashMap<BlockId, &SSABlock>, 
        visited: &mut HashSet<BlockId>,
        visiting: &mut HashSet<BlockId>) 
    -> DFSResult {
        if visited.get(node_id).is_some() {
            // If already in visited, it is finished, skip
            return DFSResult::ForwardEdge(vec![])
        } else if visiting.get(node_id).is_some() {
            // Otherwise, back edge detected
            todo!()
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
                let neigh_res = self.dfs(id, block_map, visited, visiting);
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
                let neigh_res = self.dfs(true_label, block_map, visited, visiting);
                let cond = self.structurize_operand(cond);

                match neigh_res {
                    DFSResult::ForwardEdge(mut stmt) => {
                        curr_instrs.append(&mut stmt);
                        DFSResult::ForwardEdge(curr_instrs)
                    }, 
                    DFSResult::BackEdge { pointing_to_id, mut instrs } => {
                        if pointing_to_id == *node_id {                
                            // If pointing to current block, construct
                            // a loop
                            let for_stmt = HIRStmt::While { 
                                cond: cond,
                                block: instrs,
                            };

                            curr_instrs.push(for_stmt);
                            DFSResult::ForwardEdge(curr_instrs)
                        } else {
                            // Just bubble up result for the actual back edge 
                            curr_instrs.append(&mut instrs);
                            DFSResult::BackEdge { 
                                pointing_to_id, 
                                instrs: curr_instrs,
                            }
                        }
                    },
                }
            },
        };
        // Finished visiting
        visiting.remove(node_id);

        // Mark current node as visited
        visited.insert(*node_id);

        res
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
