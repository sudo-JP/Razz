use crate::{HitRecord, Point3, Ray};

pub enum ScatterResult {
    Scattered {
        attenuation: Point3,
        scattered: Ray,
    }, 
    Absorbed,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult;
}

