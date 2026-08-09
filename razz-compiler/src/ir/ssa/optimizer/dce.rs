use std::collections::HashSet;

use crate::ir::{TempId, ssa::ssa::{SSAFunction, SSAInstruction, SSAOperand, SSAProgram}};


pub fn dce_fn(func: &mut SSAFunction) -> bool {
    let mut tracked_temp: HashSet<TempId> = HashSet::new();
    let mut add_temp_from_opr = |opr: &SSAOperand| if let SSAOperand::Temp(t) = opr {
        tracked_temp.insert(t.id);
    };

    // First pass
    for block in &func.blocks {
        for instr in &block.instrs {
            match instr {
                SSAInstruction::BinOp { lhs, rhs, .. } => {
                    add_temp_from_opr(lhs);
                    add_temp_from_opr(rhs);
                },
                SSAInstruction::UnOp { value, .. } => add_temp_from_opr(value),
                SSAInstruction::Call { args, .. } => args.iter()
                    .for_each(|arg| add_temp_from_opr(arg)),
                SSAInstruction::FieldLoad { obj, .. } => add_temp_from_opr(obj),
                SSAInstruction::FieldStore { obj, value, .. } => {
                    add_temp_from_opr(obj);
                    add_temp_from_opr(value);
                },
                SSAInstruction::Copy { value, .. } => add_temp_from_opr(value),
                SSAInstruction::Construct { fields, .. } => fields.iter()
                    .for_each(|field| add_temp_from_opr(&field.value)),
                SSAInstruction::HTTPWrite { value, .. } => add_temp_from_opr(value),
                SSAInstruction::Phi { args, .. } => args.iter()
                    .for_each(|arg| add_temp_from_opr(&arg.operand)),
                _ => {},
            }
        }
    }
    false
}

pub fn dce(ssa_prog: &mut SSAProgram) -> bool {
    ssa_prog.functions.iter_mut()
        .fold(false, |acc, mut f| acc || dce_fn(&mut f))
}
