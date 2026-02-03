use crate::{render::Image, Camera, Interval, Sample, Vec3, Viewport, World};

const MAX_DEPTH: i32 = 50;

pub struct Renderer {
    samples_per_pxl: i32,
}

impl Renderer {
    pub fn new(samples_per_pxl: i32) -> Self {
        Self { samples_per_pxl: samples_per_pxl }
    }

    pub fn render(&self, img: &mut Image, vp: &Viewport, cam: &Camera, world: &World) {
        let sam = Sample::new(cam.get_pos(), vp, img, self.samples_per_pxl);

        for pixel_index in 0..(img.width * img.height) {
            let i = (pixel_index / img.width) as f64; // row
            let j = (pixel_index % img.width) as f64; // column

            let mut color = Vec3::zeros();

            for _ in 0..self.samples_per_pxl {
                let r = cam.ray_aa(i, j, &sam);
                color = r.ray_color(world, MAX_DEPTH) + color;
            }

            //let r = cam.ray(i, j, &sam);
            //let v = r.ray_color(world);
            let v = color * sam.get_pixel_sample_scale();

            let start_index = (pixel_index * img.n) as usize;

            let intensity = Interval::new_with_val(0., 0.999);
            //let ir = (v.x() * 255.999) as u8;
            //let ig = (v.y() * 255.999) as u8;
            //let ib = (v.z() * 255.999) as u8;
            let ir = (intensity.clamp(v.x()) * 256.) as u8;
            let ig = (intensity.clamp(v.y()) * 256.) as u8;
            let ib = (intensity.clamp(v.z()) * 256.) as u8;

            img.matrix[start_index] = ir;
            img.matrix[start_index + 1] = ig;
            img.matrix[start_index + 2] = ib;
        }
    }
}
