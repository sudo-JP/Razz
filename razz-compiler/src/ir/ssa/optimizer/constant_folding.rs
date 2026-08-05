use crate::ir::ssa::{optimizer::Optimization, ssa::{SSABlock, SSAFunction, SSAInstruction, SSAOperand, SSAProgram}};

pub struct ConstantFolding;

//fn expand_bin_op(target: )

impl ConstantFolding {
    fn constant_fold_block(&self, block: &mut SSABlock) -> bool {
        let mut mutated = false;
        for instr in &block.instrs {
            mutated = mutated || match instr {
                SSAInstruction::BinOp { target, lhs, op, rhs } => {
                    match (lhs, rhs) {
                        (SSAOperand::Const(l1), SSAOperand::Const(l2)) => {
                            true
                        }
                        (_, _) => false
                    }
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
