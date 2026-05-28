use crate::{ast::{NodeId, Spanned, SpecificType}, common::Span};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum BinOpKind {
    // Arithmetic
    Add, 
    Sub, 
    Div, 
    Mult, 
    // Comparison
    Lt, 
    Le, 
    Gt, 
    Ge, 
    Eq, 
    Neq,
    // Booleans
    And, 
    Or, 
}

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"), 
            Self::Sub => write!(f, "-"), 
            Self::Div => write!(f, "/"),
            Self::Mult => write!(f, "*"), 
            Self::Lt => write!(f, "<"), 
            Self::Le => write!(f, "<="), 
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOpKind {
    /// !expr
    Not, 
    /// -expr
    Minus, 
}

impl fmt::Display for UnOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "!"), 
            Self::Minus => write!(f, "-"), 
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool), 
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(num) => write!(f, "{num}"),
            Self::Float(num) => write!(f, "{num}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum EndpointKind {
    Hittable, 
    Camera, 
    Background, 
    Image,
    Output,
}

impl fmt::Display for EndpointKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hittable => write!(f, "/hittable"), 
            Self::Camera => write!(f, "/camera"), 
            Self::Background => write!(f, "/background"), 
            Self::Image => write!(f, "/image"),
            Self::Output => write!(f, "/output")
        }
    }
}


/// Argument to function 
/// func((<name>: <expr>)*)
#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: Spanned<String>, 
    pub expr: Expr,
}

/// Struct field 
/// <key>: <value>
#[derive(Debug, PartialEq)]
pub struct StructField {
    pub key: Spanned<String>, 
    pub value: Expr,
}

/// Aliasing
pub type BinOp = Spanned<BinOpKind>;
pub type UnOp = Spanned<UnOpKind>;
pub type Endpoint = Spanned<EndpointKind>;

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    // Operations
    // <left> <op> <right>
    /// 1 + 2 
    BinOp {
        lhs: Box<Expr>,
        op: BinOp, 
        rhs: Box<Expr>
    },
    /// Unary Operation 
    /// <op><value>
    /// e.g !true
    UnOp {
        op: UnOp,
        value: Box<Expr>, 
    },
    /// Function Call
    /// <name>((<arg>)*)
    /// Function call func(name: expr) 
    /// Using a vec because cache locality
    /// is faster, better to do O(n^2) than
    /// O(n). Most people call a function 
    /// with at most 5 args anyway, only bad perf
    /// when >= 100 args
    FunctionCall {
        name: Spanned<String>, 
        args: Vec<Arg>,
    },
    /// Access JSON
    /// <obj>-><key>
    /// background->color
    FieldAccess {
        obj: Box<Expr>,
        key: Spanned<String>,
    },
    /// <name> { (<field>)* }
    /// Struct { author: "Jason Phan", program: "Razz" }
    /// Fields share the same reason with function
    /// call above, with why using vec over hash map
    StructLiteral {
        ty: SpecificType, 
        fields: Vec<StructField>,
    },
    /// Endpoint access
    /// GET <endpoint> 
    /// e.g GET /camera 
    HTTPRequest(Endpoint),
    /// Literals
    Constant(Literal),
    /// Identifer name
    Ident(String),
}
