use crate::math::Vec3;

type Point3 = Vec3;

pub struct Ray {
    origin: Point3, 
    direction: Vec3,
}

impl Ray {

    /*
     * Assuming data is moved here, creating new point and vec from
     * the array of values
     * */
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self {
            origin: origin,
            direction: dir
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Vec3 {
        let unit = self.direction.unit_vector();
        let a = 0.5 * (unit.y() + 1.); 
        Vec3::new(1., 1., 1.) * (1. - a) + Vec3::new(0.5, 0.7, 1.) * a
    }
}


