use crate::{render::Image, Camera, World};

pub struct Renderer;

impl Renderer {
    pub fn render(&self, img: &mut Image, cam: &Camera, world: &World) {

        for pixel_index in 0..(img.width * img.height) {
            let i = (pixel_index / img.width) as f64; // row
            let j = (pixel_index % img.width) as f64; // column

            let r = cam.ray(i, j);
            
            let v = r.ray_color(world);

            let start_index = (pixel_index * img.n) as usize;

            let ir = (v.x() * 255.999) as u8;
            let ig = (v.y() * 255.999) as u8;
            let ib = (v.z() * 255.999) as u8;

            img.matrix[start_index] = ir;
            img.matrix[start_index + 1] = ig;
            img.matrix[start_index + 2] = ib;
        }
    }
}
