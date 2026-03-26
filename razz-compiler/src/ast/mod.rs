use crate::{ast::statement::FnDecl, common::Span};

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
    Vec3,
    Point3,
    Color,
    Background,
    Camera,
    Output,
    Sphere,
    Image,
}

/// Top-level program node
pub struct Program {
    pub statements: Vec<FnDecl> 
}
