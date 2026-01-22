use crate::render::{Renderer, RenderError};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PPMRenderer {
    height: i32, 
    width: i32,
    path: String,
}

impl PPMRenderer {
    pub fn new(height: i32, width: i32, path: String) -> Option<Self> {
        Some(Self {
            height: height, 
            width: width, 
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
        match writeln!(writer, "P3\n{} {}\n255", self.width, self.height) {
            Ok(_) => {},
            Err(_) => { return Err(RenderError::RenderError); }
        };

        for i in 0..self.height {
            for j in 0..self.width {
                let r = (i as f32) / (self.width - 1) as f32;
                let g = 0.0;
                let b = (j as f32) / (self.height - 1) as f32;

                let ir = (r * 255.999) as i32;
                let ig = (g * 255.999) as i32;
                let ib = (b * 255.999) as i32;

                writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
            }
        } 
        writer.flush().unwrap();
        Ok(())
    }
}
