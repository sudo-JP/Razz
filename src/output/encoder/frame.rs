use crate::render::Image;
use crc::{Crc, CRC_16_XMODEM};

const MAGIC_BYTES: u16 = 0xBABE;
const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

struct FrameHeader {
    magic: u16,
    width: u16,
    height: u16,
    header_checksum: u16,
}

impl FrameHeader {
    pub fn new(magic: u16, width: u16, height: u16) -> Self {
        // 3 u16s = 6 bytes
        let mut data = Vec::with_capacity(6);
        data.extend_from_slice(&magic.to_le_bytes());
        data.extend_from_slice(&width.to_le_bytes());
        data.extend_from_slice(&height.to_le_bytes());
        let header_checksum = CRC16.checksum(&data);
        
        Self { magic, width, height, header_checksum }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8); // 4 u16s = 8 bytes

        bytes.extend_from_slice(&self.magic.to_le_bytes());
        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.header_checksum.to_le_bytes());
        bytes
    }
}



/// Due to hardware spec, i gotta use bgr instead
fn rgb888_to_bgr565(r: u8, g: u8, b: u8) -> u16 {
    let blue = (b as u16 >> 3) << 11;
    let green = (g as u16 >> 2) << 5;
    let red = r as u16 >> 3;
    blue | green | red 
}

fn rgb565_converter(img: &Image) -> Vec<u16> {
    img.matrix
        .chunks_exact(3)
        .map(|rgb| rgb888_to_bgr565(rgb[0], rgb[1], rgb[2]))
        .collect()
}

fn compress_img(img: &Image) -> Vec<u8> {
    let converted = rgb565_converter(img);
    converted.iter()
    .flat_map(|&pixel| pixel.to_le_bytes())
    .collect()
}

pub struct Encoder {
    width: u16,
    height: u16,
    payload: Vec<u8>,
    header: Option<FrameHeader>,
}


impl Encoder {
    pub fn new(img: &Image) -> Self {
        let payload = compress_img(img);
        Self {
            width: img.width as u16,
            height: img.height as u16,
            payload: payload,
            header: None,
        }
    }

    pub fn with_header(mut self) -> Encoder {
        let header = FrameHeader::new(MAGIC_BYTES, self.width, self.height);
        self.header = Some(header);
        self
    } 

    pub fn encode(self) -> Vec<u8> {
        if let Some(header) = self.header {
            let mut bytes = Vec::with_capacity(
                size_of::<FrameHeader>() + 
                self.payload.len() +
                size_of::<u16>() // For checksum
            );

            bytes.extend_from_slice(&header.to_bytes());
            bytes.extend_from_slice(&self.payload);
            bytes.extend_from_slice(&CRC16.checksum(&self.payload).to_le_bytes());

            bytes
        } else {
            let mut bytes = Vec::with_capacity(
                size_of::<u16>() * 2 + // width 
                self.payload.len()
            );

            bytes.extend_from_slice(&self.width.to_le_bytes());
            bytes.extend_from_slice(&self.height.to_le_bytes());
            bytes.extend_from_slice(&self.payload);

            bytes
        }
    }
}

