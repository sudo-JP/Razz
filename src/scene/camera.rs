use crate::{math::Point3, Sample};
use crate::{random_f64, Ray, Vec3};


pub struct Camera {
    pos: Point3,
}

impl Camera {
    pub fn new(point: Point3) -> Self {
        Self { pos: point, }
    }

    pub fn ray(&self, i: f64, j: f64, sample: &Sample) -> Ray {
        let pixel_center = sample.get_sample_pxl(i, j);
        let ray_dir = pixel_center - self.pos;
        Ray::new(self.pos, ray_dir)
    }

    pub fn ray_aa(&self, i: f64, j: f64, sample: &Sample) -> Ray {
        let pixel_center = sample.get_sample_pxl_offset(i, j, self.sample_square());
        let ray_dir = pixel_center - self.pos;
        Ray::new(self.pos, ray_dir)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.)
    }

    pub fn get_pos(&self) -> Point3 { self.pos }
}
