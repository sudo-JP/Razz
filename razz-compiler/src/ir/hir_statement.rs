use crate::{
    ast::{TypeKind, expression::EndpointKind, statement::HTTPMethodKind},
    ir::{Temp, hir::HIRFunctionParam, hir_expression::HIRExpr},
};

pub type HIRBlock = Vec<HIRStmt>;
pub struct HIRElseIf {
    pub cond: HIRExpr,
    pub body: HIRBlock,
}

pub struct HIRFunction {
    pub name: String,
    pub params: Vec<HIRFunctionParam>,
    pub block: HIRBlock,
    pub return_ty: TypeKind,
}

pub struct HIRProgram {
    pub functions: Vec<HIRFunction>,
}

// No for loop btw, its all desugared to while
pub enum HIRStmt {
    /// <temp> = <expr>
    Assign { 
        target: Temp, 
        expr: HIRExpr 
    },
    /// <obj>-><key> = <value>
    FieldStore {
        obj: HIRExpr,
        key: String,
        value: HIRExpr,
    },
    /// while <cond> <block>
    While { 
        cond: HIRExpr, 
        block: 
        HIRBlock 
    },
    /// if <cond> <body>
    /// <else_ifs>
    /// else <else_body>
    If {
        cond: HIRExpr,
        body: HIRBlock,
        else_ifs: Vec<HIRElseIf>,
        else_body: Option<HIRBlock>,
    },
    /// return <expr>
    Return(HIRExpr),
    /// <method> <ep> <body>
    HTTPRequest {
        method: HTTPMethodKind,
        ep: EndpointKind,
        body: HIRExpr,
    },
    /// Plain expression, its more so for
    /// plain function calling
    Expr(HIRExpr),
}
