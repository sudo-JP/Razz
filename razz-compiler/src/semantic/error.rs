use crate::{ast::{expression::{Arg, BinOpKind, EndpointKind, UnOpKind}, TypeKind}, common::Span};

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
    InvalidUnOp{
        ty: TypeKind, 
        op: UnOpKind,
    },
    UndefinedFunction(String),
    WrongArgCount{
        expected: usize, 
        got: usize,
    }, 
    UnknownArg(String),      
    ArgTypeMismatch{ 
        name: String, 
        expected: TypeKind, 
        got: TypeKind,
    },
    InvalidFieldAccess(TypeKind),
    InvalidFieldAccessKey(String),
    DuplicateArg(String),
}
