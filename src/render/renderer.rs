#[derive(Debug)]
pub enum RenderError {
    InvalidOutput,
    RenderError
}

pub trait Renderer {
    fn render(&self) -> Result<(), RenderError>;
}
