use std::sync::Arc;

use clap::Parser;
use razz::cli::Cli;
use razz::output::ImageOutput;
use razz::render::Image;
use razz::{Camera, Lambertian, PPMOutput, Renderer, Sphere, Vec3, Viewport, World};

type Point3 = Vec3;

fn main() {
    // Image
    let cli = Cli::parse();
    const DIMENSION: usize = 3; 
    let mut img = Image::new(cli.width, cli.height, DIMENSION);

    // Viewport
    let vp_height = 2.0;
    let vp_width = vp_height * img.get_ratio();
    let vp = Viewport::new(vp_width, vp_height, 1.0);

    // Camera
    let cam = Camera::new(Point3::new(cli.cx, cli.cy, cli.cz));

    // World
    let mut world = World::new();
    let sph1 = Sphere::new(Point3::new(0., 0., -1.), 0.5, Arc::new(Lambertian::new(Vec3::new(255., 1., 0.))));
    let sph2 = Sphere::new(Point3::new(0., -100.5, -1.), 100., Arc::new(Lambertian::new(Vec3::new(255., 1., 0.))));

    world.push(Box::new(sph1));
    world.push(Box::new(sph2));

    // Render the image, store result in img
    let renderer = Renderer::new(10);
    renderer.render(&mut img, &vp, &cam, &world);

    let output = PPMOutput::new(cli.output);
    output.write(&img).unwrap();
    /*let cam = Camera::new(Point3::new(cli.cx, cli.cy, cli.cz));
    let mut img = Image::new(cli.width, cli.height, 3, cam);
    img.generate_image_seq();
    let renderer = PPMRenderer::new(img, cli.output).unwrap();
    renderer.render().unwrap();*/
}
