use std::sync::Arc;

use clap::Parser;
use razz::cli::Cli;
use razz::output::ImageOutput;
use razz::render::Image;
use razz::vec3::Color3;
use razz::{Camera, Dielectric, Lambertian, Material, Metal, PPMOutput, Renderer, Sphere, Vec3, Viewport, World};

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
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.)));
    let material_center: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new(Color3::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material + Sync + Send> = Arc::new(Dielectric::new(1.5));
    let material_bubble: Arc<dyn Material + Sync + Send> = Arc::new(Dielectric::new(1. / 1.5));
    let material_right: Arc<dyn Material + Sync + Send> = Arc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 1.));

    let sph2 = Sphere::new(Point3::new(0., -100.5, -1.), 100., Arc::clone(&material_ground));
    let sph1 = Sphere::new(Point3::new(0., 0., -1.2), 0.5, Arc::clone(&material_center));
    let sph3 = Sphere::new(Point3::new(-1., 0., -1.), 0.5, Arc::clone(&material_left));
    let sph4 = Sphere::new(Point3::new(1., 0., -1.), 0.5, Arc::clone(&material_right));
    let sph5 = Sphere::new(Point3::new(-1., 0., -1.), 0.4, Arc::clone(&material_bubble));

    world.push(Box::new(sph1));
    world.push(Box::new(sph2));
    world.push(Box::new(sph3));
    world.push(Box::new(sph4));
    world.push(Box::new(sph5));

    // Render the image, store result in img
    let renderer = Renderer::new(10);
    renderer.cpu_render(&mut img, &vp, &cam, &world);

    let output = PPMOutput::new(cli.output);
    output.write(&img).unwrap();
}
