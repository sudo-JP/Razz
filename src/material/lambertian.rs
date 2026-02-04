use crate::{material::material::ScatterResult, HitRecord, Material, Ray, Vec3};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {

    #[allow(unused_variables)]
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let scatter_dir = {
            let dir = record.normal + Vec3::random_unit_vec();
            if dir.near_zero() { record.normal }
            else { dir }
        };
            
        let scattered = Ray::new(record.point, scatter_dir);
        let attenuation = self.albedo.clone();
        ScatterResult::Scattered { attenuation: attenuation, scattered: scattered }
    }
}
