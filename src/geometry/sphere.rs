use crate::hittable::HitSide;
use crate::Interval;
use crate::{vec3::dot, Ray, Vec3};
use crate::geometry::hittable::{HitRecord, Hittable};

pub type Point3 = Vec3;

/*
 * The root solution to hit_sphere
 * One holds the closest (-) solution
 * */
pub enum IntersectSphere {
    One(f64),  
    None
}

pub struct Sphere {
    center: Point3,
    radius: f64, 
}

impl Sphere {
    pub fn new(center: Point3, r: f64) -> Self {
        Self { center: center, radius: r.max(0.) }
    }
}


impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();

        // Quadratic formula here
        // a,b and c constants
        let a = dot(ray.direction(), ray.direction());
        let h = dot(ray.direction(), oc);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        let sqrtd = discriminant.sqrt();

        // Check the two point, starting from closest to further 
        // to be within tmin < root < tmax
        let r1 = (h - sqrtd) / a;
        let r2 = (h + sqrtd) / a; 

        let make_hit = |t| {
            let point = ray.at(t);
            let outward_norm = (point - self.center) / self.radius;
            let mut rec = HitRecord {
                point,
                // Place holders 
                normal: Vec3::zeros(),
                side: HitSide::Outside,
                t,
            };

            // set side will modify normal and side 
            rec.set_side(ray, outward_norm);
            rec
        };

        if ray_t.surrounds(r1) {
            Some(make_hit(r1))
        } else if ray_t.surrounds(r2) {
            Some(make_hit(r2))
        } else {
            None
        }
    }
}
