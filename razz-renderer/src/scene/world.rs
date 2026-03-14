use razz_core::math::{vec3::Color3, Interval, Ray};
use crate::geometry::hittable::{HitRecord, Hittable};

/// Background struct holds the lerp color representation.
/// 
pub struct Background {
    pub top: Color3, 
    pub bottom: Color3, 
}

impl Background {
    pub fn new(top: Color3, bottom: Color3) -> Self {
        Self { top, bottom }
    }

    pub fn new_normalize(top: Color3, bottom: Color3) -> Self {
        let norm_top = Color3::new(top.x() / 255., top.y() / 255., top.z() / 255.);
        let norm_bot = Color3::new(bottom.x() / 255., bottom.y() / 255., bottom.z() / 255.);
        Self { top: norm_top, bottom: norm_bot }
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
    pub bg: Background,
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_any = None; 
        let mut closest_so_far = ray_t.max;

        self.objects
            .iter()
            .for_each(|object| {
                match object.hit(ray, &Interval::new_with_val(ray_t.min, closest_so_far)) {
                    Some(rec) => {
                        closest_so_far = rec.t;
                        hit_any = Some(rec);
                    },
                    None => {}
                }
        });
        hit_any
    }
}

impl World {
    pub fn new(bg: Background) -> Self { Self { objects: vec![], bg }}

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
