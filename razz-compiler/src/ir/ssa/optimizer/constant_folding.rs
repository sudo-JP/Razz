use std::ops::{Add, Div, Mul, Sub};

use crate::{ast::expression::{BinOpKind, Literal, UnOpKind}, 
    ir::{Temp, ssa::{
        ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram}}
    }
};


fn expand_un_op_instr(target: &Temp, op: &UnOpKind, value: &SSAOperand) -> Option<SSAInstruction> {
    match value  {
        SSAOperand::Const(l) => {
            let folded = expand_un_op_literal(op, l);
            Some(SSAInstruction::Copy { target: *target, value: SSAOperand::Const(folded) })
        },
        _ => None, 
    }
}

fn expand_un_op_literal(op: &UnOpKind, value: &Literal) -> Literal {
    match (op, value) {
        (UnOpKind::Not, Literal::Bool(b)) => Literal::Bool(!*b),
        (UnOpKind::Minus, Literal::Int(i)) => Literal::Int(-*i), 
        (UnOpKind::Minus, Literal::Float(f)) => Literal::Float(-*f),
        _ => unreachable!("semantic problem")
    }
}

fn expand_bin_op_instr(target: &Temp, lhs: &SSAOperand, op: &BinOpKind, rhs: &SSAOperand) -> Option<SSAInstruction> {
    match (lhs, rhs) {
        (SSAOperand::Const(l1), SSAOperand::Const(l2)) => {
            let folded = expand_bin_op_literal(l1, op, l2);
            Some(SSAInstruction::Copy { target: *target, value: SSAOperand::Const(folded) })
        },
        (_, _) => None, 
    }
}

// This is hell to type btw 
fn expand_bin_op_literal(lhs: &Literal, op: &BinOpKind, rhs: &Literal) -> Literal {
    match (lhs, op, rhs) {
        // Int 
        // Arithmetic
        (Literal::Int(i1), BinOpKind::Add, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Sub, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Div, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Mult, Literal::Int(i2)) 
        => 
            Literal::Int(expand_bin_op_arithmetic(*i1, op, *i2)),

        // Comparison
        (Literal::Int(i1), BinOpKind::Lt, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Le, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Gt, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Ge, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Eq, Literal::Int(i2)) 
        | (Literal::Int(i1), BinOpKind::Neq, Literal::Int(i2)) 
        => 
            Literal::Bool(expand_bin_op_comparison(*i1, op, *i2)),

        // Float
        // Arithmetic
        (Literal::Float(f1), BinOpKind::Add, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Sub, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Div, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Mult, Literal::Float(f2))  
        => Literal::Float(expand_bin_op_arithmetic(*f1, op, *f2)),

        // Comparison
        (Literal::Float(f1), BinOpKind::Lt, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Le, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Gt, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Ge, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Eq, Literal::Float(f2)) 
        | (Literal::Float(f1), BinOpKind::Neq, Literal::Float(f2)) 
        => 
            Literal::Bool(expand_bin_op_comparison(*f1, op, *f2)),

        // Bool
        // Comparison
        (Literal::Bool(b1), BinOpKind::Eq, Literal::Bool(b2)) 
        | (Literal::Bool(b1), BinOpKind::Neq, Literal::Bool(b2)) 
        => 
            Literal::Bool(expand_bin_op_comparison(*b1, op, *b2)),

        (Literal::Bool(b1), BinOpKind::And, Literal::Bool(b2))  
        => Literal::Bool(*b1 && *b2),
        (Literal::Bool(b1), BinOpKind::Or, Literal::Bool(b2)) 
        => Literal::Bool(*b1 || *b2),

        (Literal::String(s1), BinOpKind::Add, Literal::String(s2))
        => Literal::String(format!("{s1}{s2}")),
        _ => unreachable!("semantic problem")
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

fn expand_bin_op_comparison<T>(lhs: T, op: &BinOpKind, rhs: T) -> bool
where 
    T: PartialOrd
    + PartialEq {
    match op {
        BinOpKind::Lt => lhs < rhs, 
        BinOpKind::Le => lhs <= rhs, 
        BinOpKind::Gt => lhs > rhs, 
        BinOpKind::Ge => lhs >= rhs, 
        BinOpKind::Eq => lhs == rhs, 
        BinOpKind::Neq => lhs != rhs, 
        _ => unreachable!("Same err as above")
    }
}

fn constant_fold_block(block: &mut SSABlock) -> bool {
    let mut mutated = false;
    for i in 0..block.instrs.len() {
        mutated = mutated || match &block.instrs[i] {
            // Bin op 
            SSAInstruction::BinOp { target, lhs, op, rhs } => {
                let instr = expand_bin_op_instr(target, lhs, op, rhs);
                let instr_mutated = instr.is_some();
                if let Some(instr) = instr {
                    block.instrs[i] = instr;
                }
                instr_mutated
            },
            // Un op
            SSAInstruction::UnOp { target, op, value } => {
                let instr = expand_un_op_instr(target, op, value);
                let instr_mutated = instr.is_some();
                if let Some(instr) = instr {
                    block.instrs[i] = instr;
                }
                instr_mutated
            },
            _ => false,
        };
    }
    mutated
}

fn constant_fold_fn(function: &mut SSAFunction) -> bool {
    function.blocks.iter_mut()
        .fold(false, |acc, mut block| acc | constant_fold_block(&mut block))
}

pub fn constant_folding(ssa_prog: &mut SSAProgram) -> bool {
    ssa_prog.functions.iter_mut()
        .fold(false, |acc, mut f| acc | constant_fold_fn(&mut f))
}
