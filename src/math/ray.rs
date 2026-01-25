use crate::{math::Vec3, Sphere};
use crate::sphere::IntersectSphere;

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

    pub fn origin(&self) -> Point3 { self.origin.clone() }
    pub fn direction(&self) -> Vec3 { self.direction.clone() }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self, sph: &Sphere) -> Vec3 {
        match sph.hit_sphere(&self) {
            IntersectSphere::One(root) => {
                let n = (self.at(root) - Vec3::new(0., 0., -1.)).unit_vector() + 1.;
                n * 0.5
            }
            IntersectSphere::None => {
                let unit = self.direction.unit_vector();
                let a = 0.5 * (unit.y() + 1.); 
                Vec3::new(1., 1., 1.) * (1. - a) + Vec3::new(0.5, 0.7, 1.) * a
            }
        }

    }
}


