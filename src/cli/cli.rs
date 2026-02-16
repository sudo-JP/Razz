use std::sync::Arc;

use crate::output::{ImageOutput, OutputError};
use crate::render::Image;
use crate::vec3::Color3;
use crate::{random_f64, random_range, ArduinoOutput, Camera, Dielectric, Lambertian, Material, Metal, PPMOutput, Point3, Renderer, Sphere, Vec3, World};
use clap::{ValueEnum,Parser};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, ValueEnum)]
pub enum RenderOutput {
    ppm,
    arduino,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub width: usize, 

    #[arg(long)]
    pub height: usize,

    //#[arg(short, long, default_value = "ppm")]
    //pub render: String,
    #[arg(long, help = "Camera position x", default_value_t)]
    pub cx: f64, 

    #[arg(long, help = "Camera position y", default_value_t)]
    pub cy: f64, 

    #[arg(long, help = "Camera position z", default_value_t)]
    pub cz: f64,

    #[arg(long, value_enum, help = "Output type", default_value_t = RenderOutput::ppm)]
    pub otype: RenderOutput, 
    // TODO: Add viewport width and height, check 
    // either or then calculate based on aspect ratio 
}

fn ray_trace(cli: &Cli) -> Image {
    // Image
    const DIMENSION: usize = 3; 
    let mut img = Image::new(cli.width, cli.height, DIMENSION);

    let vfov: f64 = 20.;

    //let lookfrom = Point3::new(cli.cx, cli.cy, cli.cz);
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let defocus_angle: f64 = 0.6;
    let focus_dist: f64 = 10.;

    // Camera
    let cam = Camera::new(
        lookfrom,
        lookat,
        vfov,
        vup,
        focus_dist, 
        defocus_angle,
        &img
    );

    // Material
    let ground_mat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian::new(Color3::new(0.5, 0.5, 0.5)));

    // World
    let mut world = World::new();
    let sph1 = Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::clone(&ground_mat));
    world.push(Box::new(sph1));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64(); 
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2, 
                b as f64 + 0.9 * random_f64()
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Arc<dyn Material + Sync + Send> = 
                        Arc::new(Lambertian::new(Color3::random() * Color3::random()));
                    let sph = Sphere::new(center, 0.2, Arc::clone(&albedo));
                    world.push(Box::new(sph));
                }
            }
            else if choose_mat < 0.95 {
                let albedo = Color3::random_range(0.5, 1.);
                let fuzz = random_range(0., 0.5);
                let metal: Arc<dyn Material + Send + Sync> = 
                    Arc::new(Metal::new(albedo, fuzz));
                let sph = Sphere::new(center, 0.2, Arc::clone(&metal));
                world.push(Box::new(sph));
            }
            else {
                let diel: Arc<dyn Material + Send + Sync> = 
                    Arc::new(Dielectric::new(1.5));
                let sph = Sphere::new(center, 0.2, Arc::clone(&diel));
                world.push(Box::new(sph));
            }
        }
    };

    let diel: Arc<dyn Material + Send + Sync> = 
        Arc::new(Dielectric::new(1.5));
    let sph = Sphere::new(Point3::new(0., 1., 0.), 1., Arc::clone(&diel));
    world.push(Box::new(sph));
    
    let lamb: Arc<dyn Material + Send + Sync> = 
        Arc::new(Lambertian::new(Color3::new(0.4, 0.2, 0.1)));
    let sph = Sphere::new(Point3::new(-4., 1., 0.), 1., Arc::clone(&lamb));
    world.push(Box::new(sph));

    let metal: Arc<dyn Material + Send + Sync> = 
        Arc::new(Metal::new(Color3::new(0.7, 0.6, 0.5), 0.));
    let sph = Sphere::new(Point3::new(4., 1., 0.), 1., Arc::clone(&metal));
    world.push(Box::new(sph));
    // Render the image, store result in img
    let renderer = Renderer::new(10);
    renderer.cpu_render(&mut img, &cam, &world);

    img
}

impl Cli {
    pub fn run(&self) {
        match self.otype {
            RenderOutput::ppm => self.ppm(ray_trace(&self)),
            RenderOutput::arduino => self.arduino(ray_trace(&self)),
        }
    }

    fn ppm(&self, img: Image) {
        let output = PPMOutput::new(self.output.clone());
        match output.write(&img) {
            Ok(_) => {}
            Err(e) => match e {
                OutputError::InvalidOutput => eprintln!("Can't create file"),
                OutputError::OutputError => eprintln!("Can't write to file"),
            }
        }
    }

    fn arduino(&self, img: Image) {
        let output = PPMOutput::new("test.ppm".to_string());
        match output.write(&img) {
            Ok(_) => {}
            Err(e) => match e {
                OutputError::InvalidOutput => eprintln!("Can't create file"),
                OutputError::OutputError => eprintln!("Can't write to file"),
            }
        }
        let output = ArduinoOutput::new(self.output.clone());
        output.stream(&img);
    }
}
