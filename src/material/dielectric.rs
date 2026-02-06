use crate::{hittable::HitSide, material::material::ScatterResult, vec3::{refract, Color3}, HitRecord, Material, Ray};

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self {
        Self { refraction_idx: refraction_idx, }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let attenuation = Color3::new(1., 1., 1.);
        let ri = match record.side {
            HitSide::Inside => self.refraction_idx,
            HitSide::Outside => 1. / self.refraction_idx,
        };

        let unit_dir = ray.direction().unit_vector();
        let refracted = refract(unit_dir, record.normal, ri);
        let scattered = Ray::new(record.point, refracted);
        ScatterResult::Scattered { attenuation: attenuation, scattered: scattered }
    }
}
