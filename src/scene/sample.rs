use crate::{render::Image, Point3, Vec3, Viewport};

pub struct Sample {
    p00: Point3,
    delta_u: Vec3, 
    delta_v: Vec3,
    pxl_samples_scale: f64,
}

impl Sample {
    pub fn new(cam_pos: Point3, vp: &Viewport, img: &Image, samples_per_pix: i32) -> Self {
        let mut p00 = cam_pos - Vec3::new(0., 0., vp.focal);

        // Find the top left corner of the viewport 
        p00 -= (vp.u() + vp.v()) / 2.;

        // Find the center of the (0, 0) sample
        let delta_u = vp.u() / img.width as f64;
        let delta_v = vp.v() / img.height as f64; 

        p00 += (delta_u + delta_v) * 0.5;

        let pxl_sam_scale = 1. / (samples_per_pix as f64);
        Self { 
            p00: p00, delta_u: delta_u, delta_v: delta_v, 
            pxl_samples_scale: pxl_sam_scale
        }
    }

    pub fn get_sample_pxl(&self, i: f64, j: f64) -> Point3 {
        self.p00 + (self.delta_v * i) + (self.delta_u * j)
    }

    pub fn get_sample_pxl_offset(&self, i: f64, j: f64, offset: Vec3) -> Point3 {
        self.p00 + (self.delta_v * (offset.y() + i)) + (self.delta_u * (offset.x() + j))
    }

    pub fn get_p00(&self) -> Point3 { self.p00 }

    pub fn get_delta_u(&self) -> Vec3 { self.delta_u }
    pub fn get_delta_v(&self) -> Vec3 { self.delta_v }
    pub fn get_pixel_sample_scale(&self) -> f64 { self.pxl_samples_scale }
}
