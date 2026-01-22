pub mod cli;
pub mod render;
pub mod math;

pub use cli::Cli;
pub use render::{PPMRenderer, Renderer};
pub use math::*;
