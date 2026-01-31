use crate::{Vec3, Point3, Viewport};

pub struct Sample {
    p00: Point3,
    delta_u: Vec3, 
    delta_v: Vec3,
}

impl Sample {
    pub fn new(cam_pos: Point3, vp: &Viewport, width: i32, height: i32) -> Self {
        let mut p00 = cam_pos - Vec3::new(0., 0., vp.focal);

        // Find the top left corner of the viewport 
        p00 -= (vp.u() + vp.v()) / 2.;

        // Find the center of the (0, 0) sample
        let delta_u = vp.u() / width as f64;
        let delta_v = vp.v() / height as f64; 

        p00 += (delta_u + delta_v) * 0.5;

        Self { p00: p00, delta_u: delta_u, delta_v: delta_v }
    }

    pub fn get_sample_pxl(&self, i: f64, j: f64) -> Point3 {
        self.p00 + (self.delta_v * i) + (self.delta_u * j)
    }

}
