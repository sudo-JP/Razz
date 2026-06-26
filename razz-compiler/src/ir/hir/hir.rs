use crate::{ast::TypeKind, ir::hir::hir_statement::HIRStmt};


#[derive(Debug)]
pub struct HIRFunctionParam {
    pub name: String, 
    pub ty: TypeKind,
}

pub type HIRBlock = Vec<HIRStmt>;
