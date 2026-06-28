use crate::{ast::{SpecificTypeKind, 
    expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, statement::HTTPMethodKind}, 
    ir::{Temp, hir::{hir::HIRBlock, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::HIRStmt}}
};


/// HIR Traveral
pub trait HIRWalkable {
    // ====== EXPRRESSION ====== 
    fn visit_expr(&mut self, expr: &HIRExpr) {
        walk_hir_expr(self, expr);
    }

    fn visit_bin_op(&mut self, 
        lhs: &HIRExpr, 
        _op: &BinOpKind, 
        rhs: &HIRExpr
    ) {
        walk_hir_expr(self, lhs);
        walk_hir_expr(self, rhs);
    }

    fn visit_un_op(&mut self, _op: &UnOpKind, value: &HIRExpr) {
        walk_hir_expr(self, value);
    }

    fn visit_expr_if(&mut self, cond: &HIRExpr, then: &HIRExpr, else_: &HIRExpr) {
        walk_hir_expr(self, cond);
        walk_hir_expr(self, then);
        walk_hir_expr(self, else_);
    }

    fn visit_fn_call(&mut self, _name: &str, args: &[HIRExpr]) {
        args.iter()
            .for_each(|arg| walk_hir_expr(self, arg));
    }

    fn visit_field_access(&mut self, obj: &HIRExpr, _key: &str) {
        walk_hir_expr(self, obj);
    }

    fn visit_struct_literal(&mut self, _ty: &SpecificTypeKind, fields: &[HIRFieldInit]) {
        fields.iter()
            .for_each(|f| walk_hir_expr(self, &f.value));
    }

    fn visit_http_get(&mut self, _ep: &EndpointKind);
    fn visit_temp(&mut self, temp: &Temp);
    fn visit_literal(&mut self, literal: &Literal);

    // ====== STATEMENT ====== 
    fn visit_stmt(&mut self, stmt: &HIRStmt) {
        walk_hir_stmt(self, stmt);
    }

    fn visit_assign(&mut self, _target: &Temp, expr: &HIRExpr) {
        walk_hir_expr(self, expr);
    }

    fn visit_field_store(&mut self, obj: &HIRExpr, _key: &str, value: &HIRExpr) {
        walk_hir_expr(self, obj);
        walk_hir_expr(self, value);
    }

    fn visit_while(&mut self, cond: &HIRExpr, block: &HIRBlock) {
        walk_hir_expr(self, cond);
        walk_hir_block(self, block);
    }

    fn visit_if_stmt(&mut self, cond: &HIRExpr, body: &HIRBlock, else_body: &HIRBlock) {
        walk_hir_expr(self, cond);
        walk_hir_block(self, body);
        walk_hir_block(self, else_body);
    }

    fn visit_return(&mut self, value: &HIRExpr) {
        walk_hir_expr(self, value);
    }

    fn visit_http_request(&mut self, _method: &HTTPMethodKind, _ep: &EndpointKind, body: &HIRExpr) {
        walk_hir_expr(self, body);
    }

    fn visit_stmt_expr(&mut self, expr: &HIRExpr) {
        walk_hir_expr(self, expr);
    }
}

pub fn walk_hir_expr<W: HIRWalkable + ?Sized>(walker: &mut W, expr: &HIRExpr) {
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

pub fn walk_hir_stmt<W: HIRWalkable + ?Sized>(walker: &mut W, stmt: &HIRStmt) {
    match stmt {
        HIRStmt::Assign { target, expr } => walker.visit_assign(target, expr),
        HIRStmt::FieldStore { obj, key, value } => walker.visit_field_store(obj, key, value),
        HIRStmt::While { cond, block } => walker.visit_while(cond, block),
        HIRStmt::If { cond, body, else_body } => walker.visit_if_stmt(cond, body, else_body),
        HIRStmt::Return(value) => walker.visit_return(value),
        HIRStmt::HTTPRequest { method, ep, body } => walker.visit_http_request(method, ep, body),
        HIRStmt::Expr(expr) => walker.visit_stmt_expr(expr),
    }
}

pub fn walk_hir_block<W: HIRWalkable + ?Sized>(walker: &mut W, block: &HIRBlock) {
    block.iter()
        .for_each(|stmt| walker.visit_stmt(stmt));
}
