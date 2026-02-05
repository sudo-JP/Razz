use crate::{material::material::ScatterResult, vec3::Color3, HitRecord, Material, Ray};

pub struct Metal {
    albedo: Color3 
}

impl Metal {
    pub fn new(albedo: Color3) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Metal {

    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected = ray.direction().reflect(record.normal);
        let scattered = Ray::new(record.point, reflected);
        let attenuation = self.albedo;
        ScatterResult::Scattered { 
            attenuation: attenuation, 
            scattered: scattered
        }
    }
}
