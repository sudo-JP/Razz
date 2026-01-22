use clap::Parser;
use razz::cli::Cli;
use razz::{PPMRenderer, Renderer};

fn main() {
    let cli = Cli::parse();
    let renderer = PPMRenderer::new(256, 256, cli.output).unwrap();
    renderer.render().unwrap();
}
