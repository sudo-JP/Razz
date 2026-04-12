use crate::ast::{expression::{Arg, BinOp, Endpoint, Expr, ExprKind, Literal, StructField, UnOp}, statement::{FnDecl, StmtKind}, Program, Spanned, SpecificType};

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

    fn visit_func_call(&mut self, _expr: &Expr, _name: &Spanned<String>, args: &Vec<Arg>) {
        args.iter()
            .for_each(|arg| walk_expr(self, &arg.expr));
    }

    fn visit_field_access(&mut self, _expr: &Expr, obj: &Expr, _key: &Spanned<String>) {
        walk_expr(self, obj);
    }

    fn visit_struct_lit(&mut self, _expr: &Expr, _ty: &SpecificType, fields: &Vec<StructField>) {
        fields.iter()
            .for_each(|field| walk_expr(self, &field.value));
    }

    // EXPR LEAF NODES 
    fn visit_get_request(&mut self, _expr: &Expr, _endpoint: &Endpoint) {}
    fn visit_constant(&mut self, _expr: &Expr, _lit: &Literal) {}
    fn visit_ident(&mut self, _expr: &Expr, _name: &str) {}

    // ====== STATEMENT ====== 
    fn visit_stmt(&mut self, stmt: &StmtKind) {
        walk_stmt(self, stmt);
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
pub fn walk_stmt<W: Walkable + ?Sized>(walker: &mut W, stmt: &StmtKind) {
    /*match stmt {
        StmtKind::Assign { expr, .. } => walker.visit_expr(&expr.node),
        StmtKind::AssignObj { target, expr } => {
            walker.visit_expr(&target.node);
            walker.visit_expr(&expr.node);
        },
        StmtKind::CompoundAssignObj { target, expr, .. } => {
            walker.visit_expr(&target.node);
            walker.visit_expr(&expr.node);
        }
        StmtKind::While { cond, body } => {
            walker.visit_expr(&cond.node);
            body.node.iter()
                .for_each(|instr| walker.visit_stmt(&instr.node));
        }
        StmtKind::If { cond, body, else_ifs, else_body } => {
            walker.visit_expr(&cond.node);
            body.node.iter()
                .for_each(|instr| walker.visit_stmt(&instr.node));
            else_ifs.iter()
                .for_each(|elif| {
                    walker.visit_expr(&elif.node.cond.node);
                    elif.node.body.node.iter()
                        .for_each(|instr| walker.visit_stmt(&instr.node));
                });
            if let Some(e) = else_body {
                e.node.iter()
                    .for_each(|instr| walker.visit_stmt(&instr.node));
            }
        }, 
        StmtKind::For { decl, cond, update , body } => {
            if let Some(decl) = decl {
                walker.visit_stmt(&decl.node);
            }
            if let Some(cond) = cond {
                walker.visit_expr(&cond.node);
            }
            update.iter()
                .for_each(|instr| walker.visit_stmt(&instr.node));
            body.node.iter()
                .for_each(|instr| walker.visit_stmt(&instr.node));
        },
        StmtKind::Return(e) => walker.visit_expr(&e.node),
        StmtKind::CompoundAssign { expr, .. } => walker.visit_expr(&expr.node),
        StmtKind::HTTPRequest { body, .. } => walker.visit_expr(&body.node),
        StmtKind::Expr(e) => walker.visit_expr(&e.node),
    }*/
}

pub fn walk_program<W: Walkable + ?Sized>(walker: &mut W, prog: &Program) {
    prog.funcs.iter()
        .for_each(|f| walker.visit_fn_decl(&f.node));
}

pub fn walk_fn_decl<W: Walkable + ?Sized>(walker: &mut W, func: &FnDecl) {
    /*func.body.node.iter()
        .for_each(|stmt| walker.visit_stmt(&stmt.node));*/
}
