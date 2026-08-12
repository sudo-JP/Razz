use crate::render::Image;
use crate::cli::RenderOutput;
pub mod ppm;
pub mod arduino;
pub mod encoder;

pub use ppm::PPMOutput;
pub use arduino::ArduinoOutput;

#[derive(Debug)]
pub enum OutputError {
    InvalidOutput,
    OutputError,
}

pub trait ImageOutput {
    fn output(&self, image: &Image) -> Result<(), OutputError>;
}

/// The `Output { type: OutputType; file: string; }` language struct, holding
/// which output format to use and where to write it. Dispatches to the
/// matching `ImageOutput` impl (`PPMOutput`/`ArduinoOutput`) via `write`.
#[derive(Clone, Debug)]
pub struct Output {
    pub ty: RenderOutput,
    pub file: String,
}

impl Output {
    pub fn new(ty: RenderOutput, file: String) -> Self {
        Self { ty, file }
    }

    pub fn set_type(&mut self, ty: RenderOutput) {
        self.ty = ty;
    }

    pub fn set_file(&mut self, file: String) {
        self.file = file;
    }

    pub fn write(&self, image: &Image) -> Result<(), OutputError> {
        match self.ty {
            RenderOutput::PPM => PPMOutput::new(self.file.clone()).output(image),
            RenderOutput::Arduino => ArduinoOutput::new(self.file.clone()).output(image),
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new(RenderOutput::PPM, "output.ppm".to_string())
    }
}
