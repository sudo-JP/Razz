use crate::{Camera, Point3, Ray, Sample, Sphere, Viewport, World};


pub struct Image {
    pub width: i32, 
    pub height: i32, 
    pub n: i32, // R^n
    pub ratio: f64,
    pub cam: Camera,
    pub matrix: Vec<u8>,
}

impl Image {
    pub fn new(width: i32, height: i32, n: i32, cam: Camera) -> Self {
        Self {
            width: width, 
            height: height, 
            ratio: width as f64 / height as f64,
            n: n, 
            cam: cam, 
            matrix: vec![0; (width * height * n) as usize],
        }
    }

    pub fn generate_image_seq(&mut self) {
        // Viewport setup
        let height = 2.0;
        let width = height * self.ratio;
        let vp = Viewport::new(width, height, 1.0);
        
        // Camera setup 
        let camera_pos = self.cam.get_pos();
        let sample = Sample::new(self.cam.get_pos(), vp, self.width, self.height);

        // World setup 
        let mut world = World::new();
        let sph1 = Sphere::new(Point3::new(0., 0., -1.), 0.5);
        let sph2 = Sphere::new(Point3::new(0., -100.5, -1.), 100.);

        world.push(Box::new(sph1));
        world.push(Box::new(sph2));

        for pixel_index in 0..(self.width * self.height) {
            let i = (pixel_index / self.width) as f64; // row
            let j = (pixel_index % self.width) as f64; // column

            let pixel_center = sample.get_sample_pxl(i, j);
            let ray_dir = pixel_center - camera_pos;
            let r = Ray::new(camera_pos, ray_dir);
            
            let v = r.ray_color(&world);

            let start_index = (pixel_index * self.n) as usize;

            let ir = (v.x() * 255.999) as u8;
            let ig = (v.y() * 255.999) as u8;
            let ib = (v.z() * 255.999) as u8;

            self.matrix[start_index] = ir;
            self.matrix[start_index + 1] = ig;
            self.matrix[start_index + 2] = ib;
        }
    }

    pub fn get_ratio(&self) -> f64 { self.ratio }
}
