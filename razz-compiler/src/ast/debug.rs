use crate::ast::{expression::Expr, statement::{FnDecl, Stmt}, traversal::{walk_expr, walk_fn_decl, walk_stmt, Walkable}, Program};

#[derive(Default)]
pub struct ASTDebug {
    indent: usize,
} 


impl Walkable for ASTDebug {
    fn visit_expr(&mut self, expr: &Expr) {
        let indent = " ".repeat(self.indent);
        println!("{indent}{:?}", expr);
        self.indent += 2; 
        walk_expr(self, expr); 
        self.indent -= 2; 
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        let indent = " ".repeat(self.indent);
        println!("{indent}{:?}", stmt);
        self.indent += 2; 
        walk_stmt(self, stmt);
        self.indent -= 2; 
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let indent = " ".repeat(self.indent);
        println!("{indent}fn {:?}", fn_decl.name);
        self.indent += 2; 
        fn_decl.body.stmts.iter()
            .for_each(|s| walk_stmt(self, &s.node));
        self.indent -= 2; 
    }

    fn visit_program(&mut self, prog: &Program) {
        println!("Program");
        self.indent += 2; 
        prog.funcs.iter().for_each(|s| walk_fn_decl(self, &s.node));
        self.indent -= 2; 
    }
}
