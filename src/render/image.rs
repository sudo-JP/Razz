pub struct Image {
    pub width: i32, 
    pub height: i32, 
    pub n: i32, // R^n
    pub ratio: f64,
    pub matrix: Vec<u8>,
}

impl Image {
    pub fn new(width: i32, height: i32, n: i32) -> Self {
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
