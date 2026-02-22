use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
// Hardware stuff
use mipidsi::Builder;
use mipidsi::models::ST7735s;
use mipidsi::interface::SpiInterface;
use embassy_rp::{spi::{Blocking, Config, Spi}};
use embassy_rp::{gpio::{Level, Output}};

// PINS
use embassy_rp::peripherals::{PIN_12, PIN_13, PIN_14, PIN_4, PIN_5, PIN_6, PIN_7, SPI0};

// Graphics
use embedded_graphics::prelude::*;
use mipidsi::TestImage;

// Misc
use static_cell::StaticCell;

type Display = mipidsi::Display<SpiInterface<'static, 
     ExclusiveDevice<Spi<'static, SPI0, Blocking>, 
     Output<'static>, NoDelay>, 
     Output<'static>>, 
     ST7735s, 
     Output<'static>>;

pub struct Renderer {
    display: Display,
    power: Output<'static>,
}

use embassy_rp::Peri;

pub struct RendererPins<'d> {
    pub pin_led: Peri<'d, PIN_12>,
    pub pin_dc: Peri<'d, PIN_13>,
    pub pin_cs: Peri<'d, PIN_5>,
    pub pin_rst: Peri<'d, PIN_14>,
    pub pin_sck: Peri<'d, PIN_6>,
    pub pin_mosi: Peri<'d, PIN_7>,
    pub pin_miso: Peri<'d, PIN_4>,
    pub spi0: Peri<'d, SPI0>,
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
    pub fn new(p: RendererPins<'static>) -> Self {
        let lcd_led = Output::new(p.pin_led, Level::Low);
        let dc = Output::new(p.pin_dc, Level::Low);
        let cs = Output::new(p.pin_cs, Level::High);
        let rst = Output::new(p.pin_rst, Level::Low);
        let mut config = Config::default();
        config.frequency = 1_000_000;

        let spi = Spi::new_blocking(
            p.spi0, 
            p.pin_sck, 
            p.pin_mosi, 
            p.pin_miso, 
            config
        );

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
