use crate::ir::ssa::{optimizer::{constant_folding::constant_folding, constant_prop::constant_propegation}, ssa::SSAProgram};
pub mod constant_folding;
pub mod constant_prop;
pub mod dce;


pub fn optimize_ssa(prog: &mut SSAProgram) {
    let mut flag = true; 
    while flag {
        flag = constant_folding(prog)
        | constant_propegation(prog);
    }
}
