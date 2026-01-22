use clap::Parser;
use razz::cli::Cli;
use razz::{PPMRenderer, Renderer};

fn main() {
    let cli = Cli::parse();
    let renderer = PPMRenderer::new(cli.width, cli.height, cli.output).unwrap();
    renderer.render().unwrap();
}
