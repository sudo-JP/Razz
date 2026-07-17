use crate::get_docs;
use std::{fs::File, io::{self, BufWriter, Write}};

use crate::ir::hir::{hir_statement::HIRProgram, traversal::HIRWalkable};

pub struct RustCodegen {
    indent: usize,
    file_writer: BufWriter<File>
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
        let a = get_docs!("//!");
        write!(self.file_writer, "{a}").unwrap();

        self.file_writer.flush().unwrap();
    }
}

impl HIRWalkable for RustCodegen {
    
}
