use crate::{Interval, Ray};
use crate::geometry::hittable::{HitRecord, Hittable};

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>
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
    pub fn new() -> Self { Self { objects: vec![] }}

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
