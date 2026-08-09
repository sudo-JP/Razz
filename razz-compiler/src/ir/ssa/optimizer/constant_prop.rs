use std::collections::HashMap;

use crate::{ast::expression::Literal, 
    ir::{Temp, TempId, ssa::ssa::{SSAFunction, SSAInstruction, SSAOperand, SSAProgram, SSATerminator}}};


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
        SSAInstruction::BinOp { lhs, rhs, .. } => {
            let a = propegate_operand(lhs, prop_map);
            let b = propegate_operand(rhs, prop_map);
            a || b 
        },
        SSAInstruction::UnOp { value, .. } => {
            propegate_operand(value, prop_map)
        },
        SSAInstruction::Call { args, .. } => {
            args.iter_mut()
                .fold(false, |acc, arg| propegate_operand(arg, prop_map) | acc) 
        },
        SSAInstruction::FieldLoad { obj, .. } => {
            propegate_operand(obj, prop_map)
        },
        SSAInstruction::FieldStore { obj, value, .. } => {
            let a = propegate_operand(value, prop_map);
            let b = propegate_operand(obj, prop_map);
            a || b 
        },
        SSAInstruction::Copy { target, value } => {
            save_temp_const(target, value, prop_map);
            propegate_operand(value, prop_map)
        },
        SSAInstruction::Construct { fields, .. } => {
            fields.iter_mut()
                .fold(false, |acc, f| propegate_operand(&mut f.value, prop_map) | acc)
        },
        SSAInstruction::HTTPWrite { value, .. } => {
            propegate_operand(value, prop_map)
        }, 
        SSAInstruction::Phi { args, .. } => {
            args.iter_mut()
                .fold(false, |acc, arg| propegate_operand(&mut arg.operand, prop_map) | acc)
        }, 
        _ => false
    }
}

fn constant_propegation_fn(func: &mut SSAFunction) -> bool {
    let mut prop_map: HashMap<TempId, Literal> = HashMap::new();
    let mut flag = false;
    for block in func.blocks.as_mut_slice() {
        for mut instr in block.instrs.as_mut_slice() {
            flag |= propegate_instr(&mut instr, &mut prop_map);
        }
        flag |= match &mut block.term {
            SSATerminator::Return(opr) => propegate_operand(opr, &mut prop_map),
            SSATerminator::IfGoto { cond, .. } => propegate_operand(cond, &mut prop_map),
            _ => false,
        };
    }
    flag
}

pub fn constant_propegation(ssa_prog: &mut SSAProgram) -> bool {
    ssa_prog.functions.iter_mut()
        .fold(false, |acc, mut f| acc | constant_propegation_fn(&mut f))
}
