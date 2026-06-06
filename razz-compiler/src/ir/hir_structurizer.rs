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
            _ => todo!()
        }
    }
}
