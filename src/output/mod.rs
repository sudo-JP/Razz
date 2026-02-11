use crate::render::Image;
pub mod ppm;
pub mod arduino;

pub use ppm::PPMOutput;
pub use arduino::ArduinoOutput;

#[derive(Debug)]
pub enum OutputError {
    InvalidOutput,
    OutputError,
}

pub trait ImageOutput {
    fn write(&self, image: &Image) -> Result<(), OutputError>;
}
