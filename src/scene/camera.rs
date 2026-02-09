use crate::render::Image;
use crate::vec3::cross;
use crate::{math::Point3};
use crate::{random_f64, Ray, Vec3};


pub struct Camera {
    pos: Point3,
    pixel00: Point3, 
    delta_u: Vec3, 
    delta_v: Vec3,

}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vfov: f64, 
        vup: Vec3,
        img: &Image
    ) -> Self {
        // Viewport from fov 
        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();

        let focal_len = (lookfrom - lookat).length();

        // Calculate u, v, w basis vectors for camera
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);

        let vp_height = 2. * h * focal_len; 
        let vp_width = 
            vp_height * img.get_ratio();

        // Camera axes
        let vp_u = u * vp_width;
        let vp_v = v * -vp_height;

        let delta_u = vp_u / img.width as f64;
        let delta_v = vp_v / img.height as f64; 

        // Upper left corner
        let mut p00 = lookfrom 
            - (w * focal_len);

        p00 -= (vp_u + vp_v) / 2.;
        p00 += (delta_u + delta_v) * 0.5;


        Self { 
            pos: lookfrom,
            pixel00: p00, 
            delta_u: delta_u, 
            delta_v: delta_v,
        }
    }

    pub fn ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = 
            self.pixel00
            + self.delta_u * j as f64
            + self.delta_v * i as f64;

        let ray_dir = pixel_center - self.pos;
        Ray::new(self.pos, ray_dir)
    }

    pub fn ray_aa(&self, i: usize, j: usize) -> Ray {
        let offset_x = random_f64() - 0.5; 
        let offset_y = random_f64() - 0.5; 

        let pixel = 
            self.pixel00 
            + self.delta_u * (j as f64 + offset_x)
            + self.delta_v * (i as f64 + offset_y);
        let ray_dir = pixel - self.pos;
        Ray::new(self.pos, ray_dir)
    }

}
