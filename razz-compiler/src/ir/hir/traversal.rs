use crate::{ast::{SpecificTypeKind, expression::{BinOpKind, EndpointKind, Literal, UnOpKind}}, 
    ir::{Temp, hir::hir_expression::{HIRExpr, HIRFieldInit}}};


/// HIR Traveral
pub trait HIRWalkable {
    // ====== EXPRRESSION ====== 
    fn visit_expr(&mut self, expr: &HIRExpr) {
        walk_expr(self, expr);
    }

    fn visit_bin_op(&mut self, 
        lhs: &HIRExpr, 
        _op: &BinOpKind, 
        rhs: &HIRExpr
    ) {
        walk_expr(self, lhs);
        walk_expr(self, rhs);
    }

    fn visit_un_op(&mut self, _op: &UnOpKind, value: &HIRExpr) {
        walk_expr(self, value);
    }

    fn visit_expr_if(&mut self, cond: &HIRExpr, then: &HIRExpr, else_: &HIRExpr) {
        walk_expr(self, cond);
        walk_expr(self, then);
        walk_expr(self, else_);
    }

    fn visit_fn_call(&mut self, _name: &str, args: &[HIRExpr]) {
        args.iter()
            .for_each(|arg| walk_expr(self, arg));
    }

    fn visit_field_access(&mut self, obj: &HIRExpr, _key: &str) {
        walk_expr(self, obj);
    }

    fn visit_struct_literal(&mut self, _ty: &SpecificTypeKind, fields: &[HIRFieldInit]) {
        fields.iter()
            .for_each(|f| walk_expr(self, &f.value));
    }

    fn visit_http_get(&mut self, _ep: &EndpointKind);
    fn visit_temp(&mut self, temp: &Temp);
    fn visit_literal(&mut self, literal: &Literal);
}

pub fn walk_expr<W: HIRWalkable + ?Sized>(walker: &mut W, expr: &HIRExpr) {
    match expr {
        HIRExpr::BinOp { lhs, op, rhs } => walker.visit_bin_op(lhs, op, rhs),
        HIRExpr::UnOp { op, value } => walker.visit_un_op(op, value),
        HIRExpr::If { cond, then, else_ } => walker.visit_expr_if(cond, then, else_),
        HIRExpr::FunctionCall { name, args } => walker.visit_fn_call(name, args),
        HIRExpr::FieldAccess { obj, key } => walker.visit_field_access(obj, key),
        HIRExpr::StructLiteral { ty, fields } => walker.visit_struct_literal(ty, fields),
        HIRExpr::HTTPRequest(ep) => walker.visit_http_get(ep),
        HIRExpr::Temp(temp) => walker.visit_temp(temp),
        HIRExpr::Const(literal) => walker.visit_literal(literal),
    } 
}
