#![no_std]
#![no_main]

// Hardware blah blah
use embassy_executor::Spawner;
use embassy_rp::spi::{Config, Spi};
use embassy_rp::gpio::{Level, Output};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_graphics::prelude::*;
use panic_halt as _;

// Screen 
use mipidsi::Builder;
use mipidsi::models::ST7735s;
use mipidsi::interface::SpiInterface;
use mipidsi::TestImage;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut lcd_led = Output::new(p.PIN_12, Level::Low);
    let dc = Output::new(p.PIN_13, Level::Low);
    let cs = Output::new(p.PIN_5, Level::High);
    let rst = Output::new(p.PIN_14, Level::Low);

    let mut config = Config::default();
    config.frequency = 1_000_000;

    let spi = Spi::new_blocking(p.SPI0, p.PIN_6, p.PIN_7, p.PIN_4, config);
    let spi = ExclusiveDevice::new_no_delay(spi, cs);

    let mut buffer = [0u8; 512];
    let di = SpiInterface::new(spi, dc, &mut buffer);

    let mut display = Builder::new(ST7735s, di)
        .reset_pin(rst)
        .display_size(128, 160)
    .init(&mut embassy_time::Delay)
        .unwrap();

    TestImage::new().draw(&mut display).unwrap();

    lcd_led.set_high();

    loop {}
}
