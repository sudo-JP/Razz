use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::render::Image;

pub struct ArduinoOutput;

const MAGIC_BYTES: u16 = 0xFEED;

fn checksum_sum(data: &[u8]) -> u16 {
    data.iter()
        .fold(0u32, |acc, &byte| acc + byte as u32) as u16
}

fn rgb888_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let red = (r as u16 >> 3) << 11;
    let green = (g as u16 >> 2) << 5;
    let blue = b as u16 >> 3;
    red | green | blue
}

fn rgb565_converter(img: &Image) -> Vec<u16> {
    img.matrix
        .chunks_exact(3)
        .map(|rgb| rgb888_to_rgb565(rgb[0], rgb[1], rgb[2]))
        .collect()
}

impl ArduinoOutput {
    pub fn stream(&self, img: &Image) -> () {
        let converted = rgb565_converter(img);
        let payload: Vec<u8> = 
            converted.iter()
            .flat_map(|&pixel| pixel.to_le_bytes())
            .collect();
    }
}
