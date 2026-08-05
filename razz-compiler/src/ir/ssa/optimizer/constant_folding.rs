use std::ops::{Add, Div, Mul, Sub};

use crate::{ast::expression::{BinOpKind, Literal}, ir::{Temp, ssa::{optimizer::Optimization, ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram}}}};

pub struct ConstantFolding;

fn expand_bin_op_instr(target: &Temp, lhs: &SSAOperand, op: &BinOpKind, rhs: &SSAOperand) -> Option<SSAInstruction> {
    match (lhs, rhs) {
        (SSAOperand::Const(l1), SSAOperand::Const(l2)) => {
            let folded = expand_literal(l1, op, l2);
            todo!()
        },
        (_, _) => None, 
    }
}

fn expand_literal(lhs: &Literal, op: &BinOpKind, rhs: &Literal) -> Literal {
    match (lhs, op, rhs) {
        (Literal::Int(i1), BinOpKind::Add, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Sub, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Div, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Mult, Literal::Int(i2)) 
        => 
            Literal::Int(expand_bin_op_arithmetic(*i1, op, *i2)),

        (Literal::Float(f1), BinOpKind::Add, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Sub, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Div, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Mult, Literal::Float(f2))  
        => Literal::Float(expand_bin_op_arithmetic(*f1, op, *f2)),
        _ => todo!()
    }
}

/// This is the most ive written with rust generic...
fn expand_bin_op_arithmetic<T>(lhs: T, op: &BinOpKind, rhs: T) -> T
where 
    T: Add<Output = T> 
    + Sub<Output = T>
    + Div<Output = T> 
    + Mul<Output = T> {
    match op {
        BinOpKind::Add => lhs + rhs, 
        BinOpKind::Sub => lhs - rhs, 
        BinOpKind::Div => lhs / rhs, 
        BinOpKind::Mult => lhs * rhs, 
        _ => unreachable!("Caller deal with it, aka me")
    }
}

impl ConstantFolding {
    fn constant_fold_block(&self, block: &mut SSABlock) -> bool {
        let mut mutated = false;
        for i in 0..block.instrs.len() {
            mutated = mutated || match &block.instrs[i] {
                SSAInstruction::BinOp { target, lhs, op, rhs } => {
                    let instr = expand_bin_op_instr(target, lhs, op, rhs);
                    let instr_mutated = instr.is_some();
                    if let Some(instr) = instr {
                        block.instrs[i] = instr;
                    }
                    instr_mutated
                },
                SSAInstruction::UnOp { target, op, value } => {
                    false
                },
                _ => false,
            };
        }

        mutated
    }

    fn constant_fold_fn(&self, function: &mut SSAFunction) -> bool {
        function.blocks.iter_mut()
            .fold(false, |acc, mut block| acc || self.constant_fold_block(&mut block))
    }
}

impl Optimization for ConstantFolding {
    fn optimize(&mut self, ssa_prog: &mut SSAProgram) -> bool {
        ssa_prog.functions.iter_mut()
            .fold(false, |acc, mut f| acc || self.constant_fold_fn(&mut f))
    }
}
