use crate::{math::Point3, render::Image, Sample, Viewport};
use crate::Ray;


pub struct Camera {
    pos: Point3,
    sample: Sample,
}

impl Camera {
    pub fn new(point: Point3, vp: Viewport, img: &Image) -> Self {
        let sam = Sample::new(point, &vp, img.width, img.height);
        Self { pos: point, sample: sam }
    }

    pub fn ray(&self, i: f64, j: f64) -> Ray {
        let pixel_center = self.sample.get_sample_pxl(i, j);
        let ray_dir = pixel_center - self.pos;
        Ray::new(self.pos, ray_dir)
    }
}
