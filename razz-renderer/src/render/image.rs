pub struct Image {
    pub width: usize, 
    pub height: usize, 
    pub n: usize, // R^n
    pub ratio: f64,
    pub matrix: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize, n: usize) -> Self {
        Self {
            width: width, 
            height: height, 
            ratio: width as f64 / height as f64,
            n: n, 
            matrix: vec![0; (width * height * n) as usize],
        }
    }


    pub fn get_ratio(&self) -> f64 { self.ratio }
}
