pub mod vec3;
pub mod img_utils;
pub mod ray;
pub mod interval;

use rand::Rng;
pub use vec3::{Vec3, Point3};
pub use img_utils::*;
pub use ray::Ray;
pub use interval::Interval;


/*
 * Utils functions math
 * */

// Returns a random f64 in [0, 1)
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
