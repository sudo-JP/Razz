use crate::{ast::{SpecificTypeKind, TypeKind, expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, statement::HTTPMethodKind}, get_docs, ir::{Temp, hir::{hir::HIRBlock, hir_expression::{HIRExpr, HIRFieldInit}, hir_statement::{HIRFunction, HIRStmt}, traversal::{walk_hir_block, walk_hir_expr, walk_hir_fn_decl, walk_hir_program, walk_hir_stmt}}}, semantic::rules::{FIELD_ACCESS_MAP, FIELD_ACCESS_MAP_ERR}};
use std::{collections::HashMap, fs::File, io::{self, BufWriter, Write}};

use crate::ir::hir::{hir_statement::HIRProgram, traversal::HIRWalkable};

pub struct RustCodegen {
    indent: usize,
    file_writer: BufWriter<File>,
    fn_def: HashMap<String, TypeKind>,
}

impl RustCodegen {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::create(path)?;
        let file_writer = BufWriter::new(file);
        Ok(Self {
            indent: 0, 
            file_writer,
            fn_def: HashMap::new(),
        })
    }

    pub fn generate(&mut self, prog: HIRProgram) {
        let docs = get_docs!("//!");
        write!(self.file_writer, "{docs}").unwrap();
        walk_hir_program(self, &prog);

        self.file_writer.flush().unwrap();
    }

    fn get_indent_str(&self) -> String {
        " ".repeat(self.indent)
    }

    /// More DFS..Find if expr is string thru nestedness
    fn is_expr_str_ty(&self, expr: &HIRExpr) -> bool {
        let mut stack: Vec<&HIRExpr> = vec![expr];

        while let Some(node) = stack.pop() {

            match node {
                // Early return
                HIRExpr::Const(c) => 
                    if let Literal::String(_) = c {
                        return true;
                    }, 
                HIRExpr::Temp(t) => {
                    if t.ty == TypeKind::String {
                        return true;
                    }
                }, 
                HIRExpr::FunctionCall { name, .. } => {
                    if let Some(ty) = self.fn_def.get(name) 
                    && *ty == TypeKind::String {
                        return true;
                    }
                },
                HIRExpr::FieldAccess { obj, key } => {
                    let err = "must pass type check for field access";
                    let HIRExpr::Temp(temp) = **obj else {
                        unreachable!("{err}")
                    };

                    let TypeKind::SpecificType(sp_ty) = temp.ty else {
                        unreachable!("{err}")
                    };
                    let map = FIELD_ACCESS_MAP.get(&sp_ty)
                        .expect(FIELD_ACCESS_MAP_ERR);

                    if let TypeKind::String = map.get(key.as_str()).expect(err) {
                        return true;
                    }
                }
                // Recursive
                HIRExpr::BinOp { lhs, rhs, .. } => {
                    stack.push(lhs);
                    stack.push(rhs);
                },  
                HIRExpr::If { then, else_, .. } => {
                    stack.push(then);
                    stack.push(else_);
                }, 
                HIRExpr::UnOp { value, .. } => {
                    stack.push(value);
                }, 
                HIRExpr::HTTPRequest(_) 
                | HIRExpr::StructLiteral { .. }
                => { return false; },
            }
        }
        false
    }
}

fn clean_str(s: &str) -> String {
    s.lines()
        .map(|line| {
            let stripped = line.trim_start_matches(|c: char| c.is_whitespace());
            format!("{}\n", stripped)
        })
        .collect()
}

impl HIRWalkable for RustCodegen {
    fn visit_program(&mut self, prog: &HIRProgram) {
        prog.functions.iter()
            .for_each(|f| {self.fn_def.insert(f.name.to_string(), f.return_ty);});

        // Imports 
        let raw_import_str = r#"
            use crate::output::{{ImageOutput}};
            use std::sync::{{LazyLock, Mutex}};
        "#;

        let clean_import_str = clean_str(raw_import_str);
        writeln!(self.file_writer, "{clean_import_str}").unwrap();

        // Const 
        let raw_const_objs = r#"
            static CAMERA: LazyLock<>

            static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| Mutex::new(
                World::new(
                    Background::new(
                    Vec3::new(0.5, 0.7, 1.0), 
                    Vec3::new(1., 1., 1.)
                    )
                )
            ));
        "#;

        let clean_const_objs = clean_str(raw_const_objs);
        writeln!(self.file_writer, "{clean_const_objs}").unwrap();

        walk_hir_program(self, prog);
    }

    fn visit_fn_decl(&mut self, fn_decl: &HIRFunction) {
        let mut params_str = String::new();
        let mut first = true; 

        for param in &fn_decl.params {
            if first {
                first = false;
            } else {
                params_str.push_str(", ");
            }
            params_str.push_str(&param.name);
            params_str.push_str(": ");
            params_str.push_str(get_rust_type(&param.ty));
        }
        writeln!(self.file_writer, "fn {}({}) -> {} {{", 
            fn_decl.name, 
            params_str, 
            get_rust_type(&fn_decl.return_ty)
        )
            .unwrap();

        walk_hir_fn_decl(self, fn_decl);

        write!(self.file_writer, "}}")
            .unwrap();
    }

    fn visit_block(&mut self, block: &HIRBlock) {
        self.indent += 4; 
        walk_hir_block(self, block); 
        self.indent -= 4;
    }

    fn visit_stmt(&mut self, stmt: &HIRStmt) {
        let indent = self.get_indent_str();
        write!(self.file_writer, "{indent}").unwrap();
        walk_hir_stmt(self, stmt);
        writeln!(self.file_writer).unwrap();
    }

    fn visit_assign(&mut self, target: &Temp, expr: &HIRExpr) {
        write!(self.file_writer, "let t{} = ", target.id)
            .unwrap();
        walk_hir_expr(self, expr);
        write!(self.file_writer, ";").unwrap();
    }

    fn visit_while(&mut self, cond: &HIRExpr, block: &HIRBlock) {
        write!(self.file_writer, "while ").unwrap();
        walk_hir_expr(self, cond);
        writeln!(self.file_writer, " {{").unwrap();
        self.visit_block(block);
        let indent = self.get_indent_str();
        write!(self.file_writer, "\n{indent}}}").unwrap();
    }

    fn visit_return(&mut self, value: &HIRExpr) {
        write!(self.file_writer, "return ").unwrap();
        walk_hir_expr(self, value);
        write!(self.file_writer, ";").unwrap();
    }

    fn visit_if_stmt(&mut self, cond: &HIRExpr, body: &HIRBlock, else_body: &HIRBlock) {
        let indent = self.get_indent_str();
        // If 
        write!(self.file_writer, "if ").unwrap();
        walk_hir_expr(self, cond);
        writeln!(self.file_writer, " {{").unwrap();

        // If body block
        self.visit_block(body);
        writeln!(self.file_writer, "{indent}}}").unwrap();

        // Check if else is empty, if it is, just return,
        // otherwise add another stmt to avoid branch miss
        // even tho source lang optimized it idc
        if else_body.is_empty() { return; }
        writeln!(self.file_writer, "{indent}else {{").unwrap();
        self.visit_block(else_body);
        writeln!(self.file_writer, "{indent}}}").unwrap();
    }

    fn visit_field_store(&mut self, obj: &HIRExpr, key: &str, value: &HIRExpr) {
        walk_hir_expr(self, obj);
        write!(self.file_writer, ".set_{key}(").unwrap();
        walk_hir_expr(self, value);
        write!(self.file_writer, ");").unwrap();
    }

    fn visit_bin_op(&mut self, 
        lhs: &HIRExpr, 
        op: &BinOpKind, 
        rhs: &HIRExpr
    )
    {
        if self.is_expr_str_ty(lhs) {
            write!(self.file_writer, "format!(\"{{}}{{}}\", ")
                .unwrap();
            walk_hir_expr(self, lhs);
            write!(self.file_writer, ", ").unwrap();
            walk_hir_expr(self, rhs);
            write!(self.file_writer, ")").unwrap();
        } else {
            write!(self.file_writer, "(").unwrap();
            walk_hir_expr(self, lhs); 
            write!(self.file_writer, " {op} ").unwrap();
            walk_hir_expr(self, rhs);
            write!(self.file_writer, ")").unwrap();
        }
    }

    fn visit_un_op(&mut self, op: &UnOpKind, value: &HIRExpr) {
        write!(self.file_writer, "{op}(").unwrap();
        walk_hir_expr(self, value);
        write!(self.file_writer, ")").unwrap();
    }

    fn visit_expr_if(&mut self, cond: &HIRExpr, then: &HIRExpr, else_: &HIRExpr) {
        write!(self.file_writer, "if ").unwrap();
        walk_hir_expr(self, cond);
        write!(self.file_writer, " {{ ").unwrap();
        walk_hir_expr(self, then);
        write!(self.file_writer, " }} else {{ ").unwrap();
        walk_hir_expr(self, else_);
        write!(self.file_writer, " }} ").unwrap();
    }

    fn visit_fn_call(&mut self, name: &str, args: &[HIRExpr]) {
       write!(self.file_writer, "{name}").unwrap();
        let mut first = true; 
        for arg in args {
            if first {
                first = false; 
            } else {
                write!(self.file_writer, ", ").unwrap();
            }
            walk_hir_expr(self, arg);
        }
        write!(self.file_writer, ")").unwrap();
    }
    
    fn visit_field_access(&mut self, obj: &HIRExpr, key: &str) {
        walk_hir_expr(self, obj); 
        write!(self.file_writer, ".get_{key}()").unwrap();
    }

    fn visit_temp(&mut self, temp: &Temp) {
        write!(self.file_writer, "t{}", temp.id).unwrap();
    }

    fn visit_literal(&mut self, literal: &Literal) {
        match literal {
            Literal::Int(i) => write!(self.file_writer, "{i}").unwrap(), 
            Literal::Float(f) => write!(self.file_writer, "{f}").unwrap(),
            Literal::String(s) => write!(self.file_writer, "{s}").unwrap(),
            Literal::Bool(b) => write!(self.file_writer, "{b}").unwrap(), 
            Literal::Null => write!(self.file_writer, "()").unwrap(),
        } 
    }

    fn visit_http_get(&mut self, ep: &EndpointKind) {
        match ep {
            // TODO: add the actual global obj, prob just UUID it or smth idk
            EndpointKind::Camera => write!(self.file_writer, ".get_camera()").unwrap(),
            EndpointKind::Image => write!(self.file_writer, ".get_image()").unwrap(),
            EndpointKind::Background => write!(self.file_writer, ".get_background()").unwrap(), 
            EndpointKind::Output => write!(self.file_writer, ".get_output()").unwrap(),
            _ => unreachable!("semantic should take care of this"),
        } 
    }

    fn visit_http_request(&mut self, method: &HTTPMethodKind, ep: &EndpointKind, body: &HIRExpr) {
        
    }

    fn visit_struct_literal(&mut self, ty: &SpecificTypeKind, fields: &[HIRFieldInit]) {
        let mut fields_str = String::new();
        for field in fields {
            fields_str.push_str(&field.name);
            fields_str.push_str(": ");
            walk_hir_expr(self, &field.value);
            fields_str.push_str(", ");
        }

        let init_str = match ty {
            SpecificTypeKind::Vec3 => format!("Vec3 {{ {fields_str} }}"),
            SpecificTypeKind::Dielectric => format!("Dielectric "),
            _ => todo!()
        };

        write!(self.file_writer, "{init_str}").unwrap();
    }
}

fn get_rust_specific_type(sp_ty: &SpecificTypeKind) -> &'static str {
    match sp_ty {
        SpecificTypeKind::Vec3 => "Vec3",
        SpecificTypeKind::Dielectric => "Dielectric",
        SpecificTypeKind::Lambertian => "Lambertian",
        SpecificTypeKind::Metal => "Metal",
        SpecificTypeKind::Point3 => "Point3",
        SpecificTypeKind::Color => "Color",
        SpecificTypeKind::Background => "Background",
        SpecificTypeKind::Camera => "Camera",
        SpecificTypeKind::Sphere => "Sphere",
        SpecificTypeKind::Image => "Image",
        SpecificTypeKind::Output => "Output",
        SpecificTypeKind::PPM => "PPM",
        SpecificTypeKind::Arduino => "Arduino",
        SpecificTypeKind::Material => "Material",
        SpecificTypeKind::OutputType => "OutputType",
    }
}

fn get_rust_type(ty: &TypeKind) -> &'static str {
    match ty {
        TypeKind::Int => "i32",
        TypeKind::Float => "f64",
        TypeKind::Bool => "bool",
        TypeKind::String => "String",
        TypeKind::Null => "()", 
        TypeKind::SpecificType(sp) => get_rust_specific_type(sp),
    }
}
