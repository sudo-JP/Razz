use crate::{ast::{expression::SpecificType, statement::FnDecl}, common::Span};

pub mod expression;
pub mod statement;
pub mod traversal;
pub mod debug;


pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

/// Language types
#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Null,
    SpecificType(SpecificType),
}

/// Top-level program node
pub struct Program {
    pub funcs: Vec<Spanned<FnDecl>>
}
