use crate::render::Image;
use crate::vec3::cross;
use crate::{math::Point3};
use crate::{random_f64, Ray, Vec3};


pub struct Camera {
    pos: Point3,
    pixel00: Point3, 
    delta_u: Vec3, 
    delta_v: Vec3,

    defocus_angle: f64,
    defocus_disk_u: Vec3, 
    defocus_disk_v: Vec3, 
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vfov: f64, 
        vup: Vec3,
        focus_dist: f64, 
        defocus_angle: f64,
        img: &Image
    ) -> Self {
        // Viewport from fov 
        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();

        // Calculate u, v, w basis vectors for camera
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);

        let vp_height = 2. * h * focus_dist;
        let vp_width = 
            vp_height * img.get_ratio();

        // Camera axes
        let vp_u = u * vp_width;
        let vp_v = v * -vp_height;

        let delta_u = vp_u / img.width as f64;
        let delta_v = vp_v / img.height as f64; 

        // Upper left corner
        let mut p00 = lookfrom 
            - (w * focus_dist);

        p00 -= (vp_u + vp_v) / 2.;
        p00 += (delta_u + delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self { 
            pos: lookfrom,
            pixel00: p00, 
            delta_u: delta_u, 
            delta_v: delta_v,
            defocus_angle: defocus_angle, 
            defocus_disk_u: defocus_disk_u,
            defocus_disk_v: defocus_disk_v,
        }
    }

    pub fn ray(&self, i: usize, j: usize) -> Ray {
        let offset_x = random_f64() - 0.5;
        let offset_y = random_f64() - 0.5;

        let pixel = self.pixel00
            + self.delta_u * (j as f64 + offset_x)
            + self.delta_v * (i as f64 + offset_y);

        let origin = if self.defocus_angle <= 0.0 {
            self.pos
        } else {
            self.defocus_disk_sample()
        };

        let dir = pixel - origin;

        Ray::new(origin, dir)
    }
     
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.pos + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }
}
