use crate::{ast::{NodeId, Spanned, SpecificType}, common::Span};

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub enum UnOpKind {
    /// !expr
    Not, 
    /// -expr
    Minus, 
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool), 
    Null,
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EndpointKind {
    Hittable, 
    Camera, 
    Background, 
    Image,
    Output,
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
