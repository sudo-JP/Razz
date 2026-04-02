use crate::ast::{expression::Expr, statement::{FnDecl, Stmt}, Program};

/// Walkable trait, used to traverse the AST tree
pub trait Walkable {
    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

    fn visit_program(&mut self, prog: &Program) {
        walk_program(self, prog);
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        walk_fn_decl(self, fn_decl);
    }
}

/// Walk on only expr 
pub fn walk_expr<W: Walkable + ?Sized>(walker: &mut W, expr: &Expr) {
    match expr {
        Expr::BinOp { left, right, .. } => {
            walker.visit_expr(&left);
            walker.visit_expr(&right);
        },
        Expr::UnOp { value, .. } => walker.visit_expr(&value),
        Expr::FunctionCall { args, .. } => args.iter()
            .for_each(|arg| walker.visit_expr(&arg.expr)),
        Expr::FieldAccess { obj, .. } => walker.visit_expr(&obj),
        Expr::StructLiteral { fields, .. } => fields.iter()
            .for_each(|field| walker.visit_expr(&field.value)),

        Expr::HTTPRequest(_) 
            | Expr::Constant(_)
            | Expr::Ident(_)
            => {},
    }
}

/// Walk on only stmt 
pub fn walk_stmt<W: Walkable + ?Sized>(walker: &mut W, stmt: &Stmt) {
    match stmt {
        Stmt::Assign { expr, .. } => walker.visit_expr(expr),
        Stmt::While { cond, body } => {
            walker.visit_expr(cond);
            body.iter()
                .for_each(|instr| walker.visit_stmt(instr));
        }
        Stmt::If { cond, body, else_ifs, else_clause } => {
            walker.visit_expr(cond);
            body.iter()
                .for_each(|instr| walker.visit_stmt(instr));
            else_ifs.iter()
                .for_each(|elif| {
                    walker.visit_expr(&elif.cond);
                    elif.body.iter()
                        .for_each(|instr| walker.visit_stmt(instr));
                });
            if let Some(stmts) = else_clause {
                stmts.iter()
                    .for_each(|instr| walker.visit_stmt(instr));
            }
        }, 
        Stmt::For { decl, cond, update , body } => {
            walker.visit_stmt(&decl);
            walker.visit_expr(cond);
            update.iter()
                .for_each(|instr| walker.visit_stmt(instr));
            body.iter()
                .for_each(|instr| walker.visit_stmt(instr));
        },
        Stmt::Return(e) => walker.visit_expr(e),
        Stmt::CompoundAssign { expr, .. } => walker.visit_expr(expr),
        Stmt::HTTPRequest { body, .. } => walker.visit_expr(body),
        Stmt::Expr(e) => walker.visit_expr(e),
    }
}

pub fn walk_program<W: Walkable + ?Sized>(walker: &mut W, prog: &Program) {
    prog.funcs.iter()
        .for_each(|f| walker.visit_fn_decl(&f.node));
}

pub fn walk_fn_decl<W: Walkable + ?Sized>(walker: &mut W, func: &FnDecl) {
    func.body.iter()
        .for_each(|stmt| walker.visit_stmt(&stmt.node));
}
