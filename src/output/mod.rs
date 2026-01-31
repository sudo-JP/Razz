use crate::render::Image;
pub mod ppm;


#[derive(Debug)]
pub enum OutputError {
    InvalidOutput,
    OutputError,
}

pub trait ImageOutput {
    fn write(&self, image: &Image) -> Result<(), OutputError>;
}
