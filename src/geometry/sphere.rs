use crate::{vec3::dot, Ray, Vec3};

pub type Point3 = Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64, 
}

impl Sphere {
    pub fn new(center: Point3, r: f64) -> Self {
        Self { center: center, radius: r }
    }

    pub fn hit_sphere(&self, ray: &Ray) -> bool {
        let oc = self.center - ray.origin();

        // a,b and c constants
        let a = dot(ray.direction(), ray.direction());
        let b = dot(ray.direction(), oc) * -2.;
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c * 4.;
        discriminant >= 0.
    }
}
