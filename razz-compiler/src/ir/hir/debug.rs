use crate::{ast::expression::BinOpKind, ir::{Temp, hir::{hir::HIRBlock, hir_expression::HIRExpr, hir_statement::{HIRFunction, HIRStmt}, traversal::{HIRWalkable, walk_hir_block, walk_hir_expr, walk_hir_fn_decl, walk_hir_stmt}}}};

pub struct HIRDebug {
    indent: usize,
}

impl HIRWalkable for HIRDebug {
    fn visit_fn_decl(&mut self, fn_decl: &HIRFunction) {
        let mut params_str = String::from("");
        let mut first = true;
        for param in &fn_decl.params {
            let formatted_param = format!("{}: {}", param.name, param.ty);
            if first {
                params_str.push_str(&formatted_param);
                first = false;
            } else {
                params_str.push_str(", ");
                params_str.push_str(&formatted_param);
            }
        }

        println!("fn {}({}) {{", fn_decl.name, params_str);
        walk_hir_fn_decl(self, fn_decl);
        println!("}}");
    }

    fn visit_block(&mut self, block: &HIRBlock) {
        self.indent += 2;
        walk_hir_block(self, block);
        self.indent -= 2;
    }

    fn visit_stmt(&mut self, stmt: &HIRStmt) {
        let indent = " ".repeat(self.indent);
        print!("{indent}");
        walk_hir_stmt(self, stmt);
        println!("");
    }

    fn visit_assign(&mut self, target: &Temp, expr: &HIRExpr) {
        print!("{target} = ");
        walk_hir_expr(self, expr);
    }

    fn visit_field_store(&mut self, obj: &HIRExpr, key: &str, value: &HIRExpr) {
        walk_hir_expr(self, obj);
        print!("->{key} = ");
        walk_hir_expr(self, value);
    }

    fn visit_while(&mut self, cond: &HIRExpr, block: &HIRBlock) {
        print!("while ");
        walk_hir_expr(self, cond);
        println!(" {{");
        walk_hir_block(self, block);
        print!("\n}}");
    }

    fn visit_if_stmt(&mut self, cond: &HIRExpr, body: &HIRBlock, else_body: &HIRBlock) {
        print!("if ");
        walk_hir_expr(self, cond);
        println!(" {{");
        walk_hir_block(self, body);
        println!("\n}} else {{");
        walk_hir_block(self, else_body);
        print!("}}");
    }
}


