use crate::ast::{expression::{Endpoint, Expr}, Type};

/// else if <cond> { <body> }
#[derive(Debug)]
pub struct ElseIf {
    pub cond: Expr, 
    pub body: Vec<Stmt>,
}

/// Function parameter 
#[derive(Debug)]
pub struct Param {
    pub name: String, 
    pub ty: Type,
}

/// Compound Assignment Operator
#[derive(Debug)]
pub enum CompoundOp {
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
#[derive(Debug)]
pub enum HTTPMethod {
    Post, 
    Put, 
    Patch, 
}

/// Function definition
/// fn <name>((<param>)*) <type> { (<body>)* }
#[derive(Debug)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type, 
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    /// Variable declartion, e.g: <name> = <expr>
    /// Assignment can be inferred, e.g foo = 5 is an int
    /// It also can be annotated, e.g foo: float = 5 is a float 
    Assign {
        name: String, 
        type_ann: Option<Type>,
        expr: Expr,
    },
    /// While condition
    /// while <cond> { <body> }
    While {
        cond: Expr, 
        body: Vec<Stmt>,
    }, 
    /// If statement
    /// 0 or more else if 
    /// 0 or 1 else 
    /// if <cond> { <body> }
    /// (<else if>)* 
    /// (<else>)?
    If {
        cond: Expr, 
        body: Vec<Stmt>,
        else_ifs: Vec<ElseIf>, 
        else_clause: Option<Vec<Stmt>>,
    },
    /// For loop statement
    /// Can have multiple expr, separated by comma
    /// for <decl>; <cond>; (<expr>|<expr>,)* { <body> }
    For {
        decl: Box<Stmt>, 
        cond: Expr, 
        update: Vec<Stmt>, 
        body: Vec<Stmt>,
    }, 
    /// Return statement, return <expr>
    Return(Expr),
    /// Compound assignment operator
    /// <name> <op> <expr> 
    /// e.g foo += (1 * 2)
    CompoundAssign {
        name: String,
        op: CompoundOp, 
        expr: Expr,
    },
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
