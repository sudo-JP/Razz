use crate::{ast::{SpecificTypeKind, expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, statement::HTTPMethodKind}, ir::{Temp, hir::{hir::HIRBlock, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRFunction, HIRStmt}, traversal::{HIRWalkable, walk_hir_block, walk_hir_expr, walk_hir_fn_decl, walk_hir_stmt}}}};

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

    fn visit_return(&mut self, value: &HIRExpr) {
        print!("return ");
        walk_hir_expr(self, value);
    }

    fn visit_http_request(&mut self, method: &HTTPMethodKind, ep: &EndpointKind, body: &HIRExpr) {
        print!("{method} {ep} ");
        walk_hir_expr(self, body);
    }

    fn visit_bin_op(&mut self, 
        lhs: &HIRExpr, 
        op: &BinOpKind, 
        rhs: &HIRExpr
    )
    {
        walk_hir_expr(self, lhs); 
        print!(" {op} ");
        walk_hir_expr(self, rhs); 
    }

    fn visit_un_op(&mut self, op: &UnOpKind, value: &HIRExpr) {
        print!("{op}");
        walk_hir_expr(self, value);
    }

    fn visit_expr_if(&mut self, cond: &HIRExpr, then: &HIRExpr, else_: &HIRExpr) {
        print!("if ");
        walk_hir_expr(self, cond);
        print!(" {{ ");
        walk_hir_expr(self, then);
        print!(" }} else {{ ");
        walk_hir_expr(self, else_);
        print!(" }}");
    }

    fn visit_fn_call(&mut self, name: &str, args: &[HIRExpr]) {
        print!("{name}(");
        let mut first = true;
        for arg in args {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            walk_hir_expr(self, arg);
        }
    }

    fn visit_field_access(&mut self, obj: &HIRExpr, key: &str) {
        walk_hir_expr(self, obj);
        print!("->{key}") 
    }

    fn visit_struct_literal(&mut self, ty: &SpecificTypeKind, fields: &[HIRFieldInit]) {
        print!("{ty} ");
        let mut first = true; 
        for field in fields {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            print!("{}: ", field.name);
            walk_hir_expr(self, &field.value);
        }
    }

    fn visit_http_get(&mut self, ep: &EndpointKind) {
        print!("GET {ep}");
    }

    fn visit_temp(&mut self, temp: &Temp) {
        print!("{temp}");
    }

    fn visit_literal(&mut self, literal: &Literal) {
        print!("{literal}");
    }
}


