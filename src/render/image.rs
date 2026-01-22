pub struct Image {
    pub width: i32, 
    pub height: i32, 
    pub n: i32, // R^n
    pub matrix: Vec<i32>,
}

impl Image {
    pub fn new(width: i32, height: i32, n: i32) -> Self {
        Self {
            width: width, 
            height: height, 
            n: n, 
            matrix: vec![0; (width * height * n) as usize],
        }
    }

    pub fn generate_image(&mut self) {
        for pixel_index in 0..(self.width * self.height) {
            let i = pixel_index / self.width; // row
            let j = pixel_index % self.width; // column

            let r = (i as f32) / (self.width - 1) as f32;
            let g = 0.0;
            let b = (j as f32) / (self.height - 1) as f32;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            let start_index = (pixel_index * self.n) as usize;
            self.matrix[start_index] = ir;
            self.matrix[start_index + 1] = ig;
            self.matrix[start_index + 2] = ib;
        }
    }
}
