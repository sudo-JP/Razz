use crate::{ast::{expression::{Endpoint, Expr}, NodeId, Spanned, Type}, common::Span};

#[derive(Debug, PartialEq)]
pub struct Stmt {
    pub id: NodeId,
    pub kind: StmtKind, 
    pub span: Span,
}

/// else if <cond> { <body> }
#[derive(Debug, PartialEq)]
pub struct ElseIf {
    pub id: NodeId, 
    pub span: Span,
    pub cond: Expr, 
    pub body: Block,
}

/// Function parameter 
#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: Spanned<String>, 
    pub ty: Type,
}

/// Compound Assignment Operator
#[derive(Debug, PartialEq)]
pub enum CompoundOpKind {
    /// `+=`
    AddE, 
    /// `-=`
    SubE, 
    /// `/=`
    DivE,
    /// `*=`
    MultE,
}

/// HTTP Method for statements
#[derive(Debug, PartialEq)]
pub enum HTTPMethodKind {
    Post, 
    Put, 
    Patch, 
}

/// Function definition
/// fn <name>((<param>)*) <type> { (<body>)* }
#[derive(Debug, PartialEq)]
pub struct FnDecl {
    pub id: NodeId,
    pub name: Spanned<String>,
    pub params: Vec<Param>,
    pub return_type: Type, 
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub id: NodeId, 
    pub stmts: Vec<Stmt>, 
    pub span: Span, 
}

/// Aliasing
pub type CompoundOp = Spanned<CompoundOpKind>;
pub type HTTPMethod = Spanned<HTTPMethodKind>;

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    /// Variable declartion, e.g: <name> = <expr>
    /// Assignment can be inferred, e.g foo = 5 is an int
    /// It also can be annotated, e.g foo: float = 5 is a float 
    Assign {
        target: Expr,
        type_ann: Option<Type>,
        expr: Expr,
    },
    /// Compound assignment operator
    /// <name> <op> <expr> 
    /// e.g foo += (1 * 2)
    CompoundAssign {
        target: Expr,
        op: CompoundOp, 
        expr: Expr,
    },
    /// While condition
    /// while <cond> { <body> }
    While {
        cond: Expr, 
        body: Block,
    }, 
    /// For loop statement
    /// Can have multiple expr, separated by comma
    /// for (<decl>)?; (<cond>)?; (<expr>|<expr>,)* { <body> }
    For {
        decl: Option<Box<Stmt>>, 
        cond: Option<Expr>, 
        update: Vec<Stmt>, 
        body: Block,
    }, 
    /// If statement
    /// 0 or more else if 
    /// 0 or 1 else 
    /// if <cond> { <body> }
    /// (<else if>)* 
    /// (<else>)?
    If {
        cond: Expr, 
        body: Block, 
        else_ifs: Vec<ElseIf>, 
        else_body: Option<Block>,
    },
    /// Return statement, return <expr>
    Return(Expr),
    /// HTTP Request statements 
    /// <method> <endpoint> <body> 
    /// PATCH /camera { xFrom: 1 }
    HTTPRequest {
        method: HTTPMethod, 
        endpoint: Endpoint,
        body: Expr, 
    },
    /// Expression as a statement 
    /// Expression that returns nothing
    /// This is for bare function call
    /// foo(), where foo() update the camera
    Expr(Expr),
}
