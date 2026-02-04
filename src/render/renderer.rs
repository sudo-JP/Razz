use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

use crate::{render::Image, Camera, Interval, Sample, Vec3, Viewport, World};

const MAX_DEPTH: i32 = 50;

pub struct Renderer {
    samples_per_pxl: i32,
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. { linear_component.sqrt() }
    else { 0. }
}

impl Renderer {
    pub fn new(samples_per_pxl: i32) -> Self {
        Self { samples_per_pxl: samples_per_pxl }
    }

    pub fn render(&self, img: &mut Image, vp: &Viewport, cam: &Camera, world: &World) {
        let sam = Sample::new(cam.get_pos(), vp, img, self.samples_per_pxl);

        //for pixel_index in 0..(img.width * img.height) {

        let width = img.width;
        img.matrix
            .par_chunks_mut(3)
            .enumerate()
            .for_each(
            |(pixel_index, pixel)| {
            let row = pixel_index / width;
            let col = pixel_index % width;

            let i = row as f64;
            let j = col as f64; 

            let mut color = Vec3::zeros();

            for _ in 0..self.samples_per_pxl {
                let r = cam.ray_aa(i, j, &sam);
                color = r.ray_color(world, MAX_DEPTH) + color;
            }

            //let r = cam.ray(i, j, &sam);
            //let v = r.ray_color(world);
            let v = color * sam.get_pixel_sample_scale();

            //let start_index = (pixel_index * img.n) as usize;

            let intensity = Interval::new_with_val(0., 0.999);
            //let ir = (v.x() * 255.999) as u8;
            //let ig = (v.y() * 255.999) as u8;
            //let ib = (v.z() * 255.999) as u8;
            let r = linear_to_gamma(v.x());
            let g = linear_to_gamma(v.y());
            let b = linear_to_gamma(v.z());

            let ir = (intensity.clamp(r) * 256.) as u8;
            let ig = (intensity.clamp(g) * 256.) as u8;
            let ib = (intensity.clamp(b) * 256.) as u8;

            pixel[0] = ir;
            pixel[1] = ig;
            pixel[2] = ib;
        });
    }
}
