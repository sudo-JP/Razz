use razz_core::math::{Ray, Vec3, Interval};
use crate::geometry::hittable::Hittable;
use crate::material::material::ScatterResult;

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 { return Vec3::zeros(); }

    match world.hit(ray, &Interval::new_with_val(0.001, std::f64::INFINITY)) {
        Some(rec) => match rec.material.scatter(ray, &rec) {
            ScatterResult::Scattered { attenuation, scattered } => {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            ScatterResult::Absorbed => Vec3::zeros(),
        },
        None => {
            let unit = ray.direction().unit_vector();
            let a = 0.5 * (unit.y() + 1.0);
            Vec3::new(1., 1., 1.) * (1. - a) + Vec3::new(0.5, 0.7, 1.0) * a
        }
    }
}
