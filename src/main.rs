use clap::Parser;
use razz::cli::Cli;
use razz::render::Image;
use razz::{PPMRenderer, Renderer};

fn main() {
    let cli = Cli::parse();
    let mut img = Image::new(cli.width, cli.height, 3);
    img.generate_image_seq();
    let renderer = PPMRenderer::new(img, cli.output).unwrap();
    renderer.render().unwrap();
}
