use crate::{ast::TypeKind, ir::hir_statement::HIRStmt};


pub struct HIRFunctionParam {
    pub name: String, 
    pub ty: TypeKind,
}

pub type HIRBlock = Vec<HIRStmt>;
