use crate::render::{Image, RenderError, Renderer};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PPMRenderer {
    img: Image,
    path: String,
}

impl PPMRenderer {
    pub fn new(img: Image, path: String) -> Option<Self> {
        Some(Self {
            img: img,
            path: path,
        }) 
    }
         
}


impl Renderer for PPMRenderer {
    fn render(&self) -> Result<(), RenderError> {
        let file = match File::create(&self.path) {
            Ok(f) => f, 
            Err(_) => { return Err(RenderError::InvalidOutput); }
        };

        let mut writer = BufWriter::new(file);
        match writeln!(writer, "P3\n{} {}\n255", self.img.width, self.img.height) {
            Ok(_) => {},
            Err(_) => { return Err(RenderError::RenderError); }
        };

        let n = self.img.n as usize;
        let length = self.img.matrix.len();
        for i in (0..length).step_by(n) {
            for j in i..i + n {
                // A bit cautious here with extra whitespace
                write!(writer, "{} ", self.img.matrix[j as usize]).unwrap();
            }
            writeln!(writer, "").unwrap();
        }

        writer.flush().unwrap();
        Ok(())
    }
}
