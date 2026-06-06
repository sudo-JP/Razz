use std::fmt;

use crate::ast::TypeKind;

pub mod basic_block;
pub mod ssa;
pub mod ssa_lowerer;
pub mod hir;
pub mod hir_expression;
pub mod hir_statement;
pub mod hir_structurizer;

pub type TempId = u32;
pub type Dest = Temp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temp {
    pub id: TempId, 
    pub ty: TypeKind,
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t{}", self.id)
    }
}

