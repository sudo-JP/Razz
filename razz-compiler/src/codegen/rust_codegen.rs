use crate::{ast::{SpecificTypeKind, TypeKind, 
    expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, 
    statement::HTTPMethodKind}, codegen::rust_preprocess::HIRRustPreprocess, get_docs, ir::{Temp, TempId, hir::{hir::HIRBlock, hir_expression::{HIRExpr, HIRFieldInit}, 
        hir_statement::{HIRFunction, HIRStmt}, 
        traversal::{walk_hir_block, walk_hir_expr, walk_hir_fn_decl, walk_hir_program, walk_hir_stmt}
    }}, semantic::rules::{FIELD_ACCESS_MAP, FIELD_ACCESS_MAP_ERR}};
use std::{collections::HashMap, fs::File, io::{self, BufWriter, Write}};

use crate::ir::hir::{hir_statement::HIRProgram, traversal::HIRWalkable};

pub struct RustCodegen {
    indent: usize,
    file_writer: BufWriter<File>,
    fn_def: HashMap<String, TypeKind>,
    is_loop: bool,
    is_main: bool,
    need_loop_mut: HashMap<TempId, bool>,
}

impl RustCodegen {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::create(path)?;
        let file_writer = BufWriter::new(file);
        Ok(Self {
            indent: 0, 
            file_writer,
            fn_def: HashMap::new(),
            need_loop_mut: HashMap::new(),
            is_loop: false,
            is_main: false,
        })
    }

    pub fn generate(&mut self, prog: HIRProgram) {
        let preprocesser = HIRRustPreprocess::default();
        let need_loop_set = preprocesser.get_mut_set(&prog);
        self.need_loop_mut = need_loop_set;
        let docs = get_docs!("//!");
        write!(self.file_writer, "{docs}").unwrap();
        self.visit_program(&prog);

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

    fn apply_body(&mut self, global: &str, body: &HIRExpr) {
        match body {
            HIRExpr::StructLiteral { fields, .. } => {
                for field in fields {
                    write!(self.file_writer, "{global}.set_{}(", field.name).unwrap();
                    walk_hir_expr(self, &field.value);
                    write!(self.file_writer, ");").unwrap();
                }
            },
            _ => {
                write!(self.file_writer, "*{global} = ").unwrap();
                walk_hir_expr(self, body);
                write!(self.file_writer, ";").unwrap();
            }
        }
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
            use razz_renderer::output::{ImageOutput};
            use std::sync::{LazyLock, Mutex};
            use razz_renderer::render::Image;
            use razz_renderer::world::Background;
            use razz_core::math::{random_f64, random_range, vec3::{Color3, Point3, Vec3}};
            use razz_renderer::{Camera, Dielectric, Lambertian, Material, Metal, PPMOutput, Renderer, Sphere, World, RenderOutput};
        "#;

        let clean_import_str = clean_str(raw_import_str);
        writeln!(self.file_writer, "{clean_import_str}").unwrap();

        // Const 
        let raw_const_objs = r#"
            static IMAGE: LazyLock<Mutex<Image>> = LazyLock::new(|| Mutex::new(Image::new(0., 0., 3)));

            static CAMERA: LazyLock<Mutex<Camera>> = LazyLock::new(|| Mutex::new(
                Camera::new(
                   Point3::default(),
                   Point3::default(),
                   Vec3::default(),
                   0.6,
                   10., 
                   &Image::new(0., 0., 3)
                )
            ));

            static BACKGROUND: LazyLock<Mutex<Background>> = LazyLock::new(|| Mutex::new(
                Background::default()
            ));

            static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| Mutex::new(
                World::new(
                    Background::default()
                )
            ));

            static OUTPUT: LazyLock<Mutex<RenderOutput>> = LazyLock::new(|| Mutex::new(
                RenderOutput::PPM
            ));

            static RENDERER: LazyLock<Mutex<Renderer>> = LazyLock::new(|| Renderer::new(50));
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

        if fn_decl.name == "main" { self.is_main = true; }

        walk_hir_fn_decl(self, fn_decl);
        self.is_main = false; 

        writeln!(self.file_writer, "}}")
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
        let assign_stmt = if let Some(is_set) = self.need_loop_mut.get(&target.id) {
            match (self.is_loop, is_set) {
                // If in a loop, and flag is already set, meaning already mut,
                // just reassign 
                (true, true) => format!("t{} = ", target.id), 

                // 1. In a loop, but the flag is not set, meaning nested loop. 
                // 2. Not in a loop but flag is set..? Shouldnt happen but its mut
                // 3. Not in a loop and not set, so mut 
                _ => {
                    self.need_loop_mut.insert(target.id, true);
                    format!("let mut t{} = ", target.id)
                }
            }
        } else {
            format!("let t{} = ", target.id)
        }; 
        write!(self.file_writer, "{assign_stmt}")
            .unwrap();
        walk_hir_expr(self, expr);
        write!(self.file_writer, ";").unwrap();
    }

    fn visit_while(&mut self, cond: &HIRExpr, block: &HIRBlock) {
        let prev_flag = self.is_loop;
        self.is_loop = true;

        write!(self.file_writer, "while ").unwrap();
        walk_hir_expr(self, cond);
        writeln!(self.file_writer, " {{").unwrap();
        self.visit_block(block);
        let indent = self.get_indent_str();
        write!(self.file_writer, "\n{indent}}}").unwrap();

        self.is_loop = prev_flag
    }

    fn visit_return(&mut self, value: &HIRExpr) {
        // Entry to call renderer
        let return_ident = if self.is_main {
            let img = "&mut *IMAGE.lock().unwrap()";
            let cam = "&*CAMERA.lock().unwrap()";
            let world = "&*WORLD.lock().unwrap()";
            let renderer_str = format!("renderer.cpu_render({img}, {cam}, {world});");
            writeln!(self.file_writer, "{renderer_str}").unwrap();
            self.get_indent_str()
        } else {
            "".to_string()
        };

        write!(self.file_writer, "{return_ident}return ").unwrap();
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
        write!(self.file_writer, " }}").unwrap();
    }

    fn visit_fn_call(&mut self, name: &str, args: &[HIRExpr]) {
       write!(self.file_writer, "{name}(").unwrap();
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
        let get_global = |obj: &'static str| format!("{obj}.lock().unwrap().clone()");
        match ep {
            EndpointKind::Camera => write!(self.file_writer, "{}", get_global("CAMERA")).unwrap(),
            EndpointKind::Image => write!(self.file_writer, "{}", get_global("IMAGE")).unwrap(),
            EndpointKind::Background => write!(self.file_writer, "{}", get_global("BACKGROUND")).unwrap(), 
            EndpointKind::Output => write!(self.file_writer, "{}", get_global("OUTPUT")).unwrap(),
            _ => unreachable!("semantic should take care of this"),
        } 
    }

    fn visit_http_request(&mut self, method: &HTTPMethodKind, ep: &EndpointKind, body: &HIRExpr) {
        let err = "semantic should take care this";
        match method {
            HTTPMethodKind::Post => match ep {
                EndpointKind::Hittable => {
                    write!(self.file_writer, "WORLD.lock().unwrap().push(Box::new(").unwrap();
                    walk_hir_expr(self, body);
                    write!(self.file_writer, "));").unwrap();
                }, 
                _ => unreachable!("{err}")
            },
            HTTPMethodKind::Put
            | HTTPMethodKind::Patch => match ep {
                EndpointKind::Camera => self.apply_body("CAMERA.lock().unwrap()", body),
                EndpointKind::Background => self.apply_body("BACKGROUND.lock().unwrap()", body),
                EndpointKind::Image => self.apply_body("IMAGE.lock().unwrap()", body),
                EndpointKind::Output => self.apply_body("OUTPUT.lock().unwrap()", body),
                _ => unreachable!("{err}")
            },
        }
    }

    /// Precondition: Must have fields already reordered
    fn visit_struct_literal(&mut self, ty: &SpecificTypeKind, fields: &[HIRFieldInit]) {
        match ty {
            SpecificTypeKind::Vec3
            | SpecificTypeKind::Point3
            | SpecificTypeKind::Color
            | SpecificTypeKind::Background
            | SpecificTypeKind::Camera
            | SpecificTypeKind::Sphere
            | SpecificTypeKind::Image
            | SpecificTypeKind::Output
            | SpecificTypeKind::Dielectric
            | SpecificTypeKind::Lambertian
            | SpecificTypeKind::Metal => {
                write!(self.file_writer, "{}::new(", get_rust_specific_type(ty)).unwrap();
                let mut first = true;
                for field in fields {
                    if first {
                        first = false;
                    } else {
                        write!(self.file_writer, ", ").unwrap();
                    }
                    walk_hir_expr(self, &field.value);
                }
                write!(self.file_writer, ")").unwrap();
            }
            SpecificTypeKind::Arduino => write!(self.file_writer, "RenderOutput::Arduino").unwrap(),
            SpecificTypeKind::PPM => write!(self.file_writer, "RenderOutput::PPM").unwrap(),
            SpecificTypeKind::Material
            | SpecificTypeKind::OutputType => unreachable!("these are generic types for compiler"),
        }
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
