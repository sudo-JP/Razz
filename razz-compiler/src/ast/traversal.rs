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
        Stmt::Assign { expr, .. } => walker.visit_expr(&expr.node),
        Stmt::AssignObj { target, expr } => {
            walker.visit_expr(&target.node);
            walker.visit_expr(&expr.node);
        },
        Stmt::CompoundAssignObj { target, expr, .. } => {
            walker.visit_expr(&target.node);
            walker.visit_expr(&expr.node);
        }
        Stmt::While { cond, body } => {
            walker.visit_expr(&cond.node);
            body.node.iter()
                .for_each(|instr| walker.visit_stmt(&instr.node));
        }
        Stmt::If { cond, body, else_ifs, else_body } => {
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
        Stmt::For { decl, cond, update , body } => {
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
        Stmt::Return(e) => walker.visit_expr(&e.node),
        Stmt::CompoundAssign { expr, .. } => walker.visit_expr(&expr.node),
        Stmt::HTTPRequest { body, .. } => walker.visit_expr(&body.node),
        Stmt::Expr(e) => walker.visit_expr(&e.node),
    }
}

pub fn walk_program<W: Walkable + ?Sized>(walker: &mut W, prog: &Program) {
    prog.funcs.iter()
        .for_each(|f| walker.visit_fn_decl(&f.node));
}

pub fn walk_fn_decl<W: Walkable + ?Sized>(walker: &mut W, func: &FnDecl) {
    func.body.node.iter()
        .for_each(|stmt| walker.visit_stmt(&stmt.node));
}
