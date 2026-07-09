use strum::Display;
use crate::{ast::statement::FnDecl, common::Span};
use std::fmt;

pub mod expression;
pub mod statement;
pub mod traversal;
pub mod debug;

/// The biggest refactor ever.
pub type NodeId = u32;

#[derive(Debug, Clone)]
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
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum TypeKind {
    Int,
    Float,
    Bool,
    String,
    Null,
    SpecificType(SpecificTypeKind),
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"), 
            Self::Float => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f,"string"),
            Self::Null => write!(f, "null"),
            Self::SpecificType(sp_ty) => write!(f, "{}", sp_ty)
        }
    }
}

impl TypeKind {
    pub fn satisfies(&self, expected: &TypeKind) -> bool {
        match (self, expected) {
            (TypeKind::SpecificType(got), TypeKind::SpecificType(exp)) => got.satisfies(exp),
            _ => self == expected,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Display)]
pub enum SpecificTypeKind {
    Dielectric, 
    Lambertian, 
    Metal,
    Vec3, 
    Point3,
    Color, 
    Background, 
    Camera, 
    Sphere, 
    Image,
    Output,
    Arduino, 
    PPM,

    // Union types 
    OutputType,
    Material,
}

impl SpecificTypeKind {
    pub fn satisfies(&self, expected: &Self) -> bool {
        match expected {
            Self::Material => matches!(self,
                Self::Dielectric 
                | Self::Lambertian
                | Self::Metal),
            Self::OutputType => matches!(self,
                Self::Arduino 
                | Self::PPM),
            other => self == other,
        }
    }
}

pub type Type = Spanned<TypeKind>;
pub type SpecificType = Spanned<SpecificTypeKind>;

/// Top-level program node
#[derive(PartialEq, Debug)]
pub struct Program {
    pub id: NodeId,
    pub funcs: Vec<Spanned<FnDecl>>
}
