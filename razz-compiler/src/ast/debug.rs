use crate::ast::{expression::Expr, statement::Stmt, traversal::{walk_expr, walk_stmt, Walkable}};

#[derive(Default)]
pub struct ASTDebug {
    ident: usize,
} 


impl Walkable for ASTDebug {
    fn visit_expr(&mut self, expr: &Expr) {
        let ident = " ".repeat(self.ident);
        println!("{}{:?}", ident, expr);
        self.ident += 2; 
        walk_expr(self, expr); 
        self.ident -= 2; 
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        let ident = " ".repeat(self.ident);
        println!("{}{:?}", ident, stmt);
        self.ident += 2; 
        walk_stmt(self, stmt);
        self.ident -= 2; 
    }
}
