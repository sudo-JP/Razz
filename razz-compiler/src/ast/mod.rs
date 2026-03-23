use crate::ast::statement::FnDecl;

pub mod expression;
pub mod statement;

pub struct Span {
    pub line: usize, 
    pub col: usize, 
}

pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

/// Language types
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
