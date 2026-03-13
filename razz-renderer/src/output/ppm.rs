use crate::output::{ImageOutput, OutputError};
use crate::render::{Image};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PPMOutput {
    path: String,
}

impl PPMOutput {
    pub fn new(path: String) -> Self {
        Self {
            path: path,
        }
    }
         
}


impl ImageOutput for PPMOutput {
    fn output(&self, img: &Image) -> Result<(), OutputError> {
        let file = match File::create(&self.path) {
            Ok(f) => f, 
            Err(_) => { return Err(OutputError::InvalidOutput); }
        };

        let mut writer = BufWriter::new(file);
        match writeln!(writer, "P3\n{} {}\n255", img.width, img.height) {
            Ok(_) => {},
            Err(_) => { return Err(OutputError::OutputError); }
        };

        let n = img.n as usize;
        let length = img.matrix.len();
        for i in (0..length).step_by(n) {
            for j in i..i + n {
                // A bit cautious here with extra whitespace
                write!(writer, "{} ", img.matrix[j as usize]).unwrap();
            }
            writeln!(writer, "").unwrap();
        }

        writer.flush().unwrap();
        Ok(())
    }
}
