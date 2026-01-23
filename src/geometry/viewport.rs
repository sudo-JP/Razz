use crate::Vec3;

type Point3 = Vec3;

pub struct Viewport {
    width: f64, 
    height: f64,
    d: f64,  
}

impl Viewport {
    pub fn new(width: f64, height: f64, focal_len: f64) -> Self {
        Self { width: width, height: height, d: -focal_len }
    }

    // V_u, horizontal vector
    fn u(&self) -> Vec3 {
        Vec3::new(-self.width, 0.0, 0.0)
    }

    // V_v, vertical vector
    fn v(&self) -> Vec3 {
        Vec3::new(0.0, self.height, 0.0)
    }

    // Unit vector u
    pub fn delta_u(&self) -> Vec3 {
        self.u() / self.width
    }

    // Unit vector v 
    pub fn delta_v(&self) -> Vec3 {
        self.v() / self.height
    }

    pub fn upper_left(&self, camera_pos: &Vec3) -> Point3 {
        // Move to the left by half
        // Move up by half 
        /*let half_width = ((self.width) / 2.) as f64;
        let half_height = ((self.height) / 2.) as f64;
        Point3::new(camera_pos.x() - half_width, camera_pos.y() + half_height, self.d)*/
        camera_pos.clone() - Vec3::new(0., 0., self.d) - self.u()/2. - self.v()/2.
    }

    pub fn p00(&self, camera_pos: &Vec3) -> Point3 {
        self.upper_left(camera_pos) + (self.delta_u() + self.delta_v()) * 0.5
    }
}
