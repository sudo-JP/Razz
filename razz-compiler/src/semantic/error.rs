use crate::{ast::{expression::{BinOpKind, EndpointKind}, TypeKind}, common::Span};

#[derive(Debug)]
pub enum SemanticError {
    UndeclaredVariable(String),
    InvalidGetRequest(EndpointKind),
    InvalidBinOp{
        ty: TypeKind, 
        op: BinOpKind,
    },
    TypeMismatch{
        expected: TypeKind, 
        got: TypeKind, 
        span: Span,
    },
}
