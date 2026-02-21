// Hardware stuff
use mipidsi::Builder;
use mipidsi::models::ST7735s;
use mipidsi::interface::SpiInterface;
use embassy_rp::{peripherals::SPI0, spi::{Blocking, Config, Spi}};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use embassy_rp::{gpio::{Level, Output}, Peripherals};

// Graphics
use embedded_graphics::prelude::*;
use mipidsi::TestImage;

// Misc
use static_cell::StaticCell;

type Display = mipidsi::Display<SpiInterface<'static, ExclusiveDevice<Spi<'static, SPI0, Blocking>, Output<'static>, NoDelay>, Output<'static>>, ST7735s, Output<'static>>;

pub struct Renderer {
    display: Display,
    power: Output<'static>,
}

impl Renderer {
    /*
     * WIRE THESE ACCORDINGLY 
     *
     * LED -> 
     * CS -> GND
     * DC -> PIN 13 
     * RST -> PIN 14
     *
     * */
    pub fn new(p: Peripherals) -> Self {
        let lcd_led = Output::new(p.PIN_12, Level::Low);
        let dc = Output::new(p.PIN_13, Level::Low);
        let cs = Output::new(p.PIN_5, Level::High);
        let rst = Output::new(p.PIN_14, Level::Low);
        let mut config = Config::default();
        config.frequency = 1_000_000;

        let spi = Spi::new_blocking(p.SPI0, p.PIN_6, p.PIN_7, p.PIN_4, config);

        let spi = ExclusiveDevice::new_no_delay(spi, cs);

        // Wrapper that lets init static once 
        static SPI_BUFFER: StaticCell<[u8; 512]> = StaticCell::new();
        let buffer = SPI_BUFFER.init([0u8; 512]);
        let di = SpiInterface::new(spi, dc, buffer);

        let display = Builder::new(ST7735s, di)
            .reset_pin(rst)
            .display_size(128, 160)
            .init(&mut embassy_time::Delay)
            .unwrap();

        Self { display: display, power: lcd_led }
    }

    pub fn on(&mut self) { self.power.set_high(); }

    pub fn draw(&mut self) { 
        TestImage::new()
            .draw(&mut self.display)
            .unwrap()
    }
}
