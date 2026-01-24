use clap::Parser;
use razz::cli::Cli;
use razz::render::Image;
use razz::{Camera, PPMRenderer, Renderer, Vec3};

type Point3 = Vec3;

fn main() {
    let cli = Cli::parse();
    let cam = Camera::new(Point3::new(cli.cx, cli.cy, cli.cz));
    let mut img = Image::new(cli.width, cli.height, 3, cam);
    img.generate_image_seq();
    let renderer = PPMRenderer::new(img, cli.output).unwrap();
    renderer.render().unwrap();
}
