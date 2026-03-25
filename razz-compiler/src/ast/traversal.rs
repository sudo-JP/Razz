use crate::ast::{expression::Expr, statement::Stmt};

/// Walkable trait, used to traverse the AST tree
pub trait Walkable {
    fn visit_expr(&self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_stmt(&self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

}

/// Walk on only expr 
pub fn walk_expr<W: Walkable +  ?Sized>(walker: &W, expr: &Expr) {
    let visit_expr = |e: &Expr| walker.visit_expr(e);
    match expr {
        Expr::BinOp { left, right, .. } => {
            visit_expr(&left);
            visit_expr(&right);
        },
        Expr::UnaryOp { value, .. } => visit_expr(&value),
        Expr::FunctionCall { args, .. } => args.iter()
            .for_each(|arg| visit_expr(&arg.expr)),
        Expr::FieldAccess { obj, .. } => visit_expr(&obj),
        Expr::StructLiteral { fields, .. } => fields.iter()
            .for_each(|field| visit_expr(&field.value)),

        Expr::HTTPRequest(_) 
            | Expr::Constant(_)
            | Expr::Identifier(_)
            => {},
    }
}

/// Walk on only stmt 
pub fn walk_stmt<W: Walkable + ?Sized>(walker: &W, stmt: &Stmt) {
    let visit_expr = |e: &Expr| walker.visit_expr(e);
    let visit_stmt = |s: &Stmt| walker.visit_stmt(s);

    match stmt {
        Stmt::Assign { expr, .. } => visit_expr(expr),
        Stmt::While { cond, body } => {
            visit_expr(cond);
            body.iter()
                .for_each(visit_stmt);
        }
        Stmt::If { cond, body, else_ifs, else_clause } => {
            walker.visit_expr(cond);
            body.iter()
                .for_each(visit_stmt);
            else_ifs.iter()
                .for_each(|elif| {
                    visit_expr(&elif.cond);
                    elif.body.iter()
                        .for_each(visit_stmt);
                });
            if let Some(stmts) = else_clause {
                stmts.iter()
                    .for_each(visit_stmt);
            }
        }, 
        Stmt::For { decl, cond, update , body } => {
            visit_stmt(&decl);
            visit_expr(cond);
            update.iter()
                .for_each(visit_stmt);
            body.iter()
                .for_each(visit_stmt);
        },
        Stmt::Return(e) => visit_expr(e),
        Stmt::FnDecl(decl) => decl.body.iter()
            .for_each(visit_stmt),
        Stmt::CompoundAssign { expr, .. } => visit_expr(expr),
        Stmt::HTTPRequest { body, .. } => visit_expr(body),
        Stmt::Expr(e) => visit_expr(e),
    }
}
