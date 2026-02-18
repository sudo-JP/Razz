use std::{time};
use owo_colors::OwoColorize;

use crate::{output::{encoding::FrameBuilder, ImageOutput, OutputError}, render::Image};

pub struct ArduinoOutput {
    path: String,
}

const SERIAL_BAUD_RATE: u32 = 115_200;

impl ArduinoOutput {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }

    fn stream_to_port(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let mut port = serialport::new(self.path.clone(), SERIAL_BAUD_RATE)
            .timeout(time::Duration::from_millis(1000))
            .open()?;

        // Waiting for connection before sending data 
        std::thread::sleep(time::Duration::from_millis(5000));
        
        port.write_all(&bytes)?;
        port.flush()?;
        Ok(())
    }
}

impl ImageOutput for ArduinoOutput {

    fn output(&self, img: &Image) -> Result<(), OutputError> {

        // Generate data bytes
        println!("{}", "Encoding the image to binary...".yellow());
        let bytes = FrameBuilder::new(img)
            .with_header()
            .build();

        // Stream data
        println!("{}", "Streaming the image to Arduino...".yellow());
        if let Err(e) = self.stream_to_port(&bytes) {
            eprintln!("{} {}", "Failed to stream".red(), e.red());
            return Err(OutputError::OutputError);
        }

        println!("{}", "Successfully streamed image to Arduino".green());
        Ok(())
    }
}
