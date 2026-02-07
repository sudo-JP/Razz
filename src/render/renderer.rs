use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

use crate::{linear_to_gamma, render::Image, Camera, Interval, Sample, Vec3, Viewport, World};

const MAX_DEPTH: i32 = 50;

pub struct Renderer {
    samples_per_pxl: i32,
}


impl Renderer {
    pub fn new(samples_per_pxl: i32) -> Self {
        Self { samples_per_pxl: samples_per_pxl }
    }

    pub fn cpu_render(&self, img: &mut Image, vp: &Viewport, cam: &Camera, world: &World) {
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

            // Sampling pixel
            for _ in 0..self.samples_per_pxl {
                let r = cam.ray_aa(i, j, &sam);
                color = r.ray_color(world, MAX_DEPTH) + color;
            }

            // Color vector
            let v = color * sam.get_pixel_sample_scale();

            // Apply gamma
            let intensity = Interval::new_with_val(0., 0.999);
            let r = linear_to_gamma(v.x());
            let g = linear_to_gamma(v.y());
            let b = linear_to_gamma(v.z());

            let ir = (intensity.clamp(r) * 256.) as u8;
            let ig = (intensity.clamp(g) * 256.) as u8;
            let ib = (intensity.clamp(b) * 256.) as u8;

            // Write color to matrix
            pixel[0] = ir;
            pixel[1] = ig;
            pixel[2] = ib;
        });
    }
}
