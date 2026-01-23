use std::sync::Arc;
use crate::{Camera, Ray, Vec3, Viewport};

type Point3 = Vec3;

pub struct Image {
    pub width: i32, 
    pub height: i32, 
    pub n: i32, // R^n
    pub ratio: f64,
    pub matrix: Vec<i32>,
}

impl Image {
    pub fn new(width: i32, height: i32, n: i32) -> Self {
        Self {
            width: width, 
            height: height, 
            ratio: width as f64 / height as f64,
            n: n, 
            matrix: vec![0; (width * height * n) as usize],
        }
    }

    pub fn generate_image_seq(&mut self) {
        let height = 2.0;
        let width = height * self.ratio;
        let viewport = Arc::new(Viewport::new(width, height, 1.0));
        let camera = Camera::new(Point3::new(0., 0., 0.), Arc::clone(&viewport));

        for pixel_index in 0..(self.width * self.height) {
            let i = (pixel_index / self.width) as f64; // row
            let j = (pixel_index % self.width) as f64; // column

            let pixel_center = viewport.p00(&camera.get_pos()) + (viewport.delta_u() * j) + (viewport.delta_v() * i);
            let ray_dir = pixel_center - camera.get_pos();
            let r = Ray::new(camera.get_pos(), ray_dir);
            
            let v = r.ray_color();

            let start_index = (pixel_index * self.n) as usize;

            let ir = (v.x() * 255.999) as i32;
            let ig = (v.y() * 255.999) as i32;
            let ib = (v.z() * 255.999) as i32;

            self.matrix[start_index] = ir;
            self.matrix[start_index + 1] = ig;
            self.matrix[start_index + 2] = ib;

            //let r = (i as f32) / (self.width - 1) as f32;
            /*let r = 0.0;
            let g = 1.0;
            let b = (j as f32) / (self.height - 1) as f32;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            let start_index = (pixel_index * self.n) as usize;
            self.matrix[start_index] = ir;
            self.matrix[start_index + 1] = ig;
            self.matrix[start_index + 2] = ib;*/
        }
    }

    pub fn get_ratio(&self) -> f64 { self.ratio }
}
