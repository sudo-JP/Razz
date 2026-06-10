//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use std::collections::{HashMap, HashSet};

use crate::ir::{basic_block::BlockId, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRProgram, HIRStmt}, ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram, SSATerminator}
};


pub struct HIRStructurizer {
    curr_instrs: Vec<HIRStmt>,
    program: HIRProgram,
}

impl HIRStructurizer {
    pub fn new() -> Self {
        Self {
            curr_instrs: vec![],
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
        ancestors: &mut HashMap<BlockId, BlockId>) 
    -> Vec<HIRStmt> {
        // If already in visited, must be a loop
        if visited.get(node_id).is_some() {
            return vec![];
        }

        // Mark current node as visited
        visited.insert(*node_id);

        // Get node neighbour
        let node = block_map.get(node_id).unwrap();
        let stmts = match &node.term {
            // Base case
            SSATerminator::Return(_) => vec![],

            // Recursive case
            SSATerminator::Goto(id) => {
                ancestors.insert(*id, *node_id);
                self.dfs(id, block_map, visited, ancestors) 
            }
            SSATerminator::IfGoto { true_label, false_label, .. } => {
                ancestors.insert(*true_label, *node_id);
                ancestors.insert(*false_label, *node_id);
                self.dfs(true_label, block_map, visited, ancestors)
            },
        };

        stmts
    }

    fn structurize_operand(&self, operand: &SSAOperand) -> HIRExpr {
        match operand {
            SSAOperand::Temp(t) => HIRExpr::Temp(*t),
            SSAOperand::Const(c) => HIRExpr::Const(c.clone()),
        }
    }

    fn structurize_instr(&mut self, ssa_instr: &SSAInstruction) {
        match ssa_instr {
            SSAInstruction::BinOp { target, lhs, op, rhs } => {
                let lhs = Box::new(self.structurize_operand(lhs));
                let rhs = Box::new(self.structurize_operand(rhs));
                let binop = HIRExpr::BinOp{ 
                    lhs,
                    op: *op,
                    rhs, 
                };
                let assignment = HIRStmt::Assign { target: *target, expr: binop };
                self.curr_instrs.push(assignment);
            }, 
            SSAInstruction::UnOp { target, op, value } => {
                let value = Box::new(self.structurize_operand(value));
                let unop = HIRExpr::UnOp{ 
                    op: *op, 
                    value 
                };
                let assignment = HIRStmt::Assign { target: *target, expr: unop };
                self.curr_instrs.push(assignment);
            },
            SSAInstruction::Call { target, args, func } => {
                let fn_call = HIRExpr::FunctionCall{ 
                    name: func.to_string(), 
                    args: args.iter()
                        .map(|arg| self.structurize_operand(arg))
                        .collect(),
                };
                if let Some(t) = target {
                    let assignment = HIRStmt::Assign { target: *t, expr: fn_call };
                    self.curr_instrs.push(assignment);
                } else {
                    self.curr_instrs.push(HIRStmt::Expr(fn_call));
                }
            },
            SSAInstruction::FieldLoad { target, obj, key } => {
                let obj = Box::new(self.structurize_operand(obj));
                let struct_access = HIRExpr::FieldAccess{ 
                    obj: obj, 
                    key: key.to_string() 
                };
                let assignment = HIRStmt::Assign { target: *target, expr: struct_access };
                self.curr_instrs.push(assignment);
            },
            SSAInstruction::FieldStore { obj, key, value } => {
                let obj = self.structurize_operand(obj);
                let value = self.structurize_operand(value);
                let field_store = HIRStmt::FieldStore{ 
                    obj, 
                    key: key.to_string(), 
                    value, 
                };
                self.curr_instrs.push(field_store);
            },
            SSAInstruction::Copy { target, value } => {
                let expr = self.structurize_operand(value);
                
                let assignment = HIRStmt::Assign { target: *target, expr };
                self.curr_instrs.push(assignment);
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

                let assignment = HIRStmt::Assign { target: *target, expr: struct_lit };
                self.curr_instrs.push(assignment);
            },
            SSAInstruction::HTTPGet { target, ep } => {
                let expr = HIRExpr::HTTPRequest(*ep);

                let assignment = HIRStmt::Assign { target: *target, expr };
                self.curr_instrs.push(assignment);
            },
            SSAInstruction::HTTPWrite { method, ep, value } => {
                let body = self.structurize_operand(value);

                let req = HIRStmt::HTTPRequest { method: *method, ep: *ep, body };
                self.curr_instrs.push(req);
            },
            SSAInstruction::Phi { .. } => {},
        }
    }
}
