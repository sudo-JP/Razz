pub mod cli;
pub mod render;
pub mod math;
pub mod geometry;
pub mod scene;
pub mod output;
pub mod material;

pub use cli::Cli;
pub use render::{Renderer};
pub use math::*;
pub use geometry::*;
pub use scene::*;
pub use output::ppm::*;
pub use material::*;
