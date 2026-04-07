use crate::{ast::statement::FnDecl, common::Span};

pub mod expression;
pub mod statement;
pub mod traversal;
pub mod debug;


#[derive(Debug)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

/// Language types
#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Null,
    SpecificType(SpecificType),
}

#[derive(Debug, PartialEq)]
pub enum SpecificType {
    Dielectric, 
    Lambertian, 
    Metal,
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
#[derive(PartialEq, Debug)]
pub struct Program {
    pub funcs: Vec<Spanned<FnDecl>>
}
