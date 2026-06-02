//! This is for SSA -> HIR 
//! SSA are for optimizing the code, whereas HIR is 
//! for mapping to codegen more cleanly. 
//! Since my target languages doesn't have goto, like 
//! Rust or Python, this pass is needed

use crate::ir::ssa::SSAProgram;


pub struct HIRStructurizer {

}

impl HIRStructurizer {
    pub fn structurize(self, ssa_prog: SSAProgram) {

    }
}
