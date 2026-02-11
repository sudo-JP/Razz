use std::sync::Arc;

use clap::Parser;
use razz::cli::Cli;
use razz::output::ImageOutput;
use razz::render::Image;
use razz::vec3::Color3;
use razz::{random_f64, random_range, ArduinoOutput, Camera, Dielectric, Lambertian, Material, Metal, PPMOutput, Renderer, Sphere, Vec3, World};

type Point3 = Vec3;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
