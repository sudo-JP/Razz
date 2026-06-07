//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use crate::ir::{hir_expression::HIRExpr, hir_statement::{HIRProgram, HIRStmt}, 
    ssa::{SSAInstruction, SSAOperand, SSAProgram}
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

    pub fn structurize(self, ssa_prog: SSAProgram) -> HIRProgram {
        todo!()
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
            }
            _ => todo!()
        }
    }
}
