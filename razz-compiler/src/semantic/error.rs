use crate::{ast::{expression::{BinOpKind, EndpointKind}, TypeKind}, common::Span};

#[derive(Debug)]
pub struct SemanticError {
    pub kind: SemanticErrorKind, 
    pub span: Span,
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    UndeclaredVariable(String),
    InvalidGetRequest(EndpointKind),
    InvalidBinOp{
        ty: TypeKind, 
        op: BinOpKind,
    },
    TypeMismatch{
        expected: TypeKind, 
        got: TypeKind, 
    },
}
