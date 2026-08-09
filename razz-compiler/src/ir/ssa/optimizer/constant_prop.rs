use std::collections::HashMap;

use crate::{ast::expression::Literal, 
    ir::{Temp, TempId, ssa::ssa::{SSAFunction, SSAInstruction, SSAOperand, SSAProgram}}};


fn save_temp_const(target: &Temp, operand: &SSAOperand, prop_map: &mut HashMap<TempId, Literal>) {
    match operand {
        SSAOperand::Const(c) => {
            prop_map.insert(target.id, c.clone());
        },
        _ => {},
    }
}

fn propegate_operand(operand: &mut SSAOperand, prop_map: &mut HashMap<TempId, Literal>) -> bool {
    match operand {
        SSAOperand::Temp(t) => {
            if let Some(c) = prop_map.get(&t.id) {
                *operand = SSAOperand::Const(c.clone());
                true
            } else { false }
        }
        _ => false,
    }
}
 

fn propegate_instr(instr: &mut SSAInstruction, prop_map: &mut HashMap<TempId, Literal>) -> bool {
    match instr {
        SSAInstruction::Copy { target, value } => {
            save_temp_const(target, value, prop_map);
            propegate_operand(value, prop_map)
        },
        SSAInstruction::BinOp { lhs, rhs, .. } => {
            let a = propegate_operand(lhs, prop_map);
            let b = propegate_operand(rhs, prop_map);
            a || b 
        },
        SSAInstruction::UnOp { value, .. } => {
            propegate_operand(value, prop_map)
        },
        SSAInstruction::FieldLoad { obj, .. } => {
            propegate_operand(obj, prop_map)
        },
        SSAInstruction::FieldStore { obj, value, .. } => {
            let a = propegate_operand(value, prop_map);
            let b = propegate_operand(obj, prop_map);
            a || b 
        },
        _ => false
    }
}

fn constant_propegation_fn(func: &mut SSAFunction) -> bool {
    let mut prop_map: HashMap<TempId, Literal> = HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            match instr {
                SSAInstruction::Copy { target, value } => {
                }
                _ => {}
            } 
        }
    }
    false
}

pub fn constant_propegation(ssa_prog: &mut SSAProgram) -> bool {
    ssa_prog.functions.iter_mut()
        .fold(false, |acc, mut f| acc || constant_propegation_fn(&mut f))
}
