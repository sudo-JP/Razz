use crate::{hittable::HitSide, material::material::ScatterResult, random_f64, vec3::{dot, refract, Color3}, HitRecord, Material, Ray};

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
        
        // Reflect or refrac 
        let cos_theta = dot(unit_dir * -1., record.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1. || reflectance(cos_theta, ri) > random_f64() {
            unit_dir.reflect(record.normal)
        } else {
            refract(unit_dir, record.normal, ri)
        };

        let scattered = Ray::new(record.point, direction);
        ScatterResult::Scattered { attenuation: attenuation, scattered: scattered }
    }

}
fn reflectance(cosine: f64, refraction_idx: f64) -> f64 {
    let mut r0 = (1. - refraction_idx) / (1. + refraction_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
