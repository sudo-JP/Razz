use crate::vec3::dot;
use crate::{Interval, Ray};
use crate::math::{Point3, Vec3};

pub enum HitSide {
    Inside, 
    Outside, 
}


pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3, 
    pub t: f64,
    pub side: HitSide,
}

impl HitRecord {
    pub fn set_side(&mut self, ray: &Ray, outward_norm: Vec3) {
        let angle = dot(ray.direction(), outward_norm);
        self.side = if angle < 0. {
            self.normal = outward_norm;
            HitSide::Outside
        } else {
            self.normal = outward_norm * -1.;
            HitSide::Inside
        }

    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
