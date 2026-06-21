//! This looks pretty much like src/ast/expression.rs
use crate::{ast::{expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, SpecificTypeKind}, ir::Temp};

#[derive(Debug)]
pub struct HIRFieldInit {
    pub name: String, 
    pub value: HIRExpr,
}

#[derive(Debug)]
pub enum HIRExpr {
    /// <lhs> <op> <rhs>
    BinOp {
        lhs: Box<HIRExpr>,
        op: BinOpKind,
        rhs: Box<HIRExpr>,
    },
    /// If as an expression, body and else both 
    /// returning something 
    /// if <cond> {
    ///    return <then>
    /// } else {
    ///    return <else_>
    /// }
    If {
        cond: Box<HIRExpr>, 
        then: Box<HIRExpr>,
        else_: Box<HIRExpr>,
    },
    /// <op> <value>
    UnOp {
        op: UnOpKind, 
        value: Box<HIRExpr>,
    },
    /// <name>(<args>)
    FunctionCall {
        name: String, 
        args: Vec<HIRExpr>,
    },
    /// <obj>-><key>
    FieldAccess {
        obj: Box<HIRExpr>, 
        key: String,
    },
    /// <ty> { <fields> }
    StructLiteral {
        ty: SpecificTypeKind, 
        fields: Vec<HIRFieldInit>,
    },
    /// GET <ep>
    HTTPRequest(EndpointKind),
    /// t
    Temp(Temp),
    /// true | 1 | 1.2 etc
    Const(Literal),
}
