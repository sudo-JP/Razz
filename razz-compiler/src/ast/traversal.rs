use crate::ast::{expression::{Arg, BinOp, Endpoint, Expr, ExprKind, Literal, StructField, UnOp}, statement::{Block, CompoundOp, ElseIf, FnDecl, HTTPMethod, Stmt, StmtKind}, Program, Spanned, SpecificType, Type};

/// Walkable trait, used to traverse the AST tree
pub trait Walkable {
    // ====== EXPRRESSION ====== 
    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_bin_op(&mut self, _expr: &Expr, lhs: &Expr, _op: &BinOp, rhs: &Expr) {
        walk_expr(self, lhs);
        walk_expr(self, rhs);
    }

    fn visit_un_op(&mut self, _expr: &Expr, _op: &UnOp, value: &Expr) {
        walk_expr(self, value);
    }

    fn visit_func_call(&mut self, _expr: &Expr, _name: &Spanned<String>, args: &[Arg]) {
        args.iter()
            .for_each(|arg| walk_expr(self, &arg.expr));
    }

    fn visit_field_access(&mut self, _expr: &Expr, obj: &Expr, _key: &Spanned<String>) {
        walk_expr(self, obj);
    }

    fn visit_struct_lit(&mut self, _expr: &Expr, _ty: &SpecificType, fields: &[StructField]) {
        fields.iter()
            .for_each(|field| walk_expr(self, &field.value));
    }

    // EXPR LEAF NODES 
    fn visit_get_request(&mut self, _expr: &Expr, _endpoint: &Endpoint) {}
    fn visit_constant(&mut self, _expr: &Expr, _lit: &Literal) {}
    fn visit_ident(&mut self, _expr: &Expr, _name: &str) {}

    // ====== STATEMENT ====== 
    fn visit_stmt(&mut self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_assign(&mut self, _stmt: &Stmt, _name: &Spanned<String>, _ty: &Option<Type>, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_compound_assign(&mut self, _stmt: &Stmt, _name: &Spanned<String>, _op: &CompoundOp, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_assign_obj(&mut self, _stmt: &Stmt, target: &Expr, expr: &Expr) {
        walk_expr(self, target);
        walk_expr(self, expr);
    }

    fn visit_compound_assign_obj(&mut self, _stmt: &Stmt, target: &Expr, _op: &CompoundOp, expr: &Expr) {
        walk_expr(self, target);
        walk_expr(self, expr);
    }

    fn visit_while(&mut self, _stmt: &Stmt, cond: &Expr, body: &Block) {
        walk_expr(self, cond);
        body.stmts
            .iter()
            .for_each(|s| walk_stmt(self, s));
    }

    fn visit_if(&mut self, _stmt: &Stmt, cond: &Expr, body: &Block, else_ifs: &[ElseIf], else_body: &Option<Block>) {
        walk_expr(self, cond);
        body.stmts
            .iter()
            .for_each(|s| walk_stmt(self, s));

        for elif in else_ifs {
            walk_expr(self, &elif.cond);
            elif.body.stmts
                .iter()
                .for_each(|s| walk_stmt(self, s));
        }

        if let Some(block) = else_body {
            block.stmts
                .iter()
                .for_each(|s| walk_stmt(self, s));

        }
    }

    fn visit_for(&mut self, _stmt: &Stmt, decl: &Option<Box<Stmt>>, cond: &Option<Expr>, update: &[Stmt], body: &Block) {
        if let Some(d) = decl {
            walk_stmt(self, d);
        }

        if let Some(c) = cond {
            walk_expr(self, c);
        }

        update.iter()
            .for_each(|upd| walk_stmt(self, upd));

        body.stmts
            .iter()
            .for_each(|s| walk_stmt(self, s));
    }

    fn visit_return(&mut self, _stmt: &Stmt, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_http_request(&mut self, _stmt: &Stmt, _method: &HTTPMethod, _endpoint: &Endpoint, body: &Expr) {
        walk_expr(self, body);
    }

    fn visit_stmt_expr(&mut self, _stmt: &Stmt, expr: &Expr) {
        walk_expr(self, expr);
    }

    // General programs and fn decl
    fn visit_program(&mut self, prog: &Program) {
        walk_program(self, prog);
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        walk_fn_decl(self, fn_decl);
    }
}

/// Walk on only expr 
pub fn walk_expr<W: Walkable + ?Sized>(walker: &mut W, expr: &Expr) {
    match &expr.kind {
        ExprKind::BinOp { lhs, op, rhs } => walker.visit_bin_op(expr, &lhs, op, &rhs),
        ExprKind::UnOp { op, value } => walker.visit_un_op(expr, op, &value),

        ExprKind::FunctionCall { name, args } => walker.visit_func_call(expr, name, args),
        ExprKind::FieldAccess { obj, key } => walker.visit_field_access(expr, &obj, key),
        ExprKind::StructLiteral { ty, fields } => walker.visit_struct_lit(expr, ty, fields),

        // Leaf
        ExprKind::HTTPRequest(endpoint) => walker.visit_get_request(expr, endpoint),
        ExprKind::Constant(lit) => walker.visit_constant(expr, lit),
        ExprKind::Ident(name) => walker.visit_ident(expr, &name),
    }
}

/// Walk on only stmt 
pub fn walk_stmt<W: Walkable + ?Sized>(walker: &mut W, stmt: &Stmt) {
    match &stmt.kind {
        StmtKind::Assign { name, type_ann, expr } => walker.visit_assign(stmt, name, type_ann, expr),
        StmtKind::CompoundAssign { name, op, expr } => walker.visit_compound_assign(stmt, name, op, expr),
        StmtKind::AssignObj { target, expr } => walker.visit_assign_obj(stmt, target, expr),
        StmtKind::CompoundAssignObj { target, op, expr } => walker.visit_compound_assign_obj(stmt, target, op, expr),

        StmtKind::While { cond, body } => walker.visit_while(stmt, cond, body),
        StmtKind::For { decl, cond, update , body } => walker.visit_for(stmt, decl, cond, update, body),

        StmtKind::If { cond, body, else_ifs, else_body } => walker.visit_if(stmt, cond, body, else_ifs, else_body),

        StmtKind::Return(e) => walker.visit_return(stmt, e),
        StmtKind::HTTPRequest { method, endpoint, body } => walker. visit_http_request(stmt, method, endpoint, body),

        StmtKind::Expr(e) => walker.visit_stmt_expr(stmt, e),
    }
}

pub fn walk_program<W: Walkable + ?Sized>(walker: &mut W, prog: &Program) {
    walker.visit_program(prog);
    prog.funcs
        .iter()
        .for_each(|f| walker.visit_fn_decl(&f.node));
}

pub fn walk_fn_decl<W: Walkable + ?Sized>(walker: &mut W, func: &FnDecl) {
    func.body.stmts
        .iter()
        .for_each(|stmt| walker.visit_stmt(stmt));
}
