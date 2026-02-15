use crc::{Crc, CRC_16_XMODEM};
use std::{time};

use crate::render::Image;

pub struct ArduinoOutput {
    path: String,
}

const MAGIC_BYTES: u16 = 0xBABE;
const SERIAL_BAUD_RATE: u32 = 115_200;
const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

struct ArduinoFrameHeader {
    magic: u16,
    width: u16,
    height: u16,
    header_checksum: u16,
}

struct ArduinoFrame {
    header: ArduinoFrameHeader,
    payload: Vec<u8>,
    payload_checksum: u16, 
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

impl ArduinoFrameHeader {
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

impl ArduinoFrame {
    pub fn new(width: u16, height: u16, payload: Vec<u8>) -> Self {
        let header = ArduinoFrameHeader::new(MAGIC_BYTES, width, height);
        let payload_checksum = CRC16.checksum(&payload);
        Self {
            header: header,
            payload_checksum: payload_checksum,
            payload: payload
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            size_of::<ArduinoFrameHeader>() + self.payload.len() + 2
        );
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.payload_checksum.to_le_bytes());

        bytes
    }
}

impl ArduinoOutput {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }
    fn compress_img(&self, img: &Image) -> Vec<u8> {
        let converted = rgb565_converter(img);
        converted.iter()
        .flat_map(|&pixel| pixel.to_le_bytes())
        .collect()
    }

    fn encode(&self, img: &Image, payload: Vec<u8>) -> Vec<u8> {
        let frame = ArduinoFrame::new(
            img.width as u16, 
            img.height as u16, 
            payload
        );
        frame.to_bytes()
    }

    fn stream_to_port(&self, bytes: Vec<u8>) -> anyhow::Result<()> {
        let mut port = serialport::new(self.path.clone(), SERIAL_BAUD_RATE)
            .timeout(time::Duration::from_millis(1000))
            .open()?;

        
        port.write_all(&bytes)?;
        println!("Successfully wrote {} bytes", bytes.len());
        port.flush()?;
        Ok(())
    }

    pub fn stream(&self, img: &Image) -> () {
        // Convert to smaller byte format
        let payload = self.compress_img(img);

        // Generate data bytes
        let bytes = self.encode(img, payload);

        // Stream data
        if let Err(e) = self.stream_to_port(bytes) {
            eprintln!("Failed to stream: {}", e);
        }
    }
}
