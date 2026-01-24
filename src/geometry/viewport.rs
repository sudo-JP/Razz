use crate::Vec3;

pub struct Viewport {
    pub width: f64, 
    pub height: f64,
    pub focal: f64,  
}

impl Viewport {
    pub fn new(width: f64, height: f64, focal_len: f64) -> Self {
        Self { width: width, height: height, focal: focal_len }
    }

    // V_u, horizontal vector
    pub fn u(&self) -> Vec3 {
        Vec3::new(self.width, 0.0, 0.0)
    }

    // V_v, vertical vector
    pub fn v(&self) -> Vec3 {
        Vec3::new(0.0, -self.height, 0.0)
    }

}
