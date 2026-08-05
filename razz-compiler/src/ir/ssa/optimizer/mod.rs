use crate::ir::ssa::ssa::SSAProgram;
pub mod constant_folding;

pub trait Optimization {
    fn optimize(&mut self, ssa_prog: &mut SSAProgram) -> bool;
}
