use crate::ast::{Program, expression::Expr, statement::{Block, FnDecl, Stmt}, traversal::{ASTWalkable, walk_block, walk_expr, walk_fn_decl, walk_stmt}};

#[derive(Default)]
pub struct ASTDebug {
    indent: usize,
} 
impl ASTDebug {
    fn get_ident_str(&self) -> String {
        " ".repeat(self.indent)
    }
}

/// Who am i kidding, ill do it after
impl ASTWalkable for ASTDebug {
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let mut params_str = String::from("");
        let mut first = true;
        for param in &fn_decl.params {
            let formatted_param = format!("{}: {}", param.name.node, param.ty.node);
            if first {
                params_str.push_str(&formatted_param);
                first = false;
            } else {
                params_str.push_str(", ");
                params_str.push_str(&formatted_param);
            }
        }

        println!("fn {}({}) {} {{", fn_decl.name.node, params_str, fn_decl.return_type.node);
        walk_fn_decl(self, fn_decl);
        println!("}}");
    }

    fn visit_block(&mut self, block: &Block) {
        self.indent += 2;
        walk_block(self, block);
        self.indent -= 2;
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        let indent = self.get_ident_str();
        print!("{indent}");
        walk_stmt(self, stmt);
        println!("");
    }

    /*fn visit_assign(&mut self, stmt: &Stmt, target: &Expr, ty: &Option<Type>, expr: &Expr) {
        //print!("{target} = ");
    }*/

    fn visit_expr(&mut self, expr: &Expr) {
        let indent = " ".repeat(self.indent);
        println!("{indent}{:?}", expr);
        self.indent += 2; 
        walk_expr(self, expr); 
        self.indent -= 2; 
    }


    fn visit_program(&mut self, prog: &Program) {
        println!("Program");
        self.indent += 2; 
        prog.funcs.iter().for_each(|s| walk_fn_decl(self, &s.node));
        self.indent -= 2; 
    }
}
