use crate::{vec3::dot, Ray, Vec3};

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
        Self { center: center, radius: r }
    }

    pub fn hit_sphere(&self, ray: &Ray) -> IntersectSphere {
        let oc = self.center - ray.origin();

        // Quadratic formula here
        // a,b and c constants
        let a = dot(ray.direction(), ray.direction());
        let h = dot(ray.direction(), oc);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant >= 0. {
            IntersectSphere::One((h - discriminant.sqrt()) / a)
        } 
        else {
            IntersectSphere::None
        }
    }
}
