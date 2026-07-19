use crate::{ast::{SpecificTypeKind, TypeKind}, get_docs, ir::hir::{hir::HIRBlock, hir_statement::{HIRFunction, HIRStmt}, traversal::{walk_hir_block, walk_hir_fn_decl, walk_hir_program, walk_hir_stmt}}};
use std::{fs::File, io::{self, BufWriter, Write}};

use crate::ir::hir::{hir_statement::HIRProgram, traversal::HIRWalkable};

pub struct RustCodegen {
    indent: usize,
    file_writer: BufWriter<File>
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
        _ => todo!()
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

impl RustCodegen {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::create(path)?;
        let file_writer = BufWriter::new(file);
        Ok(Self {
            indent: 0, 
            file_writer,
        })
    }

    pub fn generate(&mut self, prog: HIRProgram) {
        let docs = get_docs!("//!");
        write!(self.file_writer, "{docs}").unwrap();
        walk_hir_program(self, &prog);

        self.file_writer.flush().unwrap();
    }

    fn get_ident_str(&self) -> String {
        " ".repeat(self.indent)
    }
}

impl HIRWalkable for RustCodegen {
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
        let indent = self.get_ident_str();
        write!(self.file_writer, "{indent}").unwrap();
        walk_hir_stmt(self, stmt);
        writeln!(self.file_writer).unwrap();
    }
}
