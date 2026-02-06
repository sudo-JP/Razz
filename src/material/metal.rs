use crate::{material::material::ScatterResult, vec3::Color3, HitRecord, Material, Ray, Vec3};

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Self {
        Self { 
            albedo: albedo, 
            fuzz: if fuzz < 1. { fuzz } else { 1. }, 
        }
    }
}

impl Material for Metal {

    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let mut reflected = ray.direction().reflect(record.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vec() * self.fuzz);
        let scattered = Ray::new(record.point, reflected);
        let attenuation = self.albedo;
        ScatterResult::Scattered { 
            attenuation: attenuation, 
            scattered: scattered
        }
    }
}
