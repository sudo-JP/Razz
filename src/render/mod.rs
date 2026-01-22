pub mod ppm;
pub mod renderer;
pub mod image;

pub use ppm::{PPMRenderer};
pub use renderer::{Renderer, RenderError};
pub use image::*;
