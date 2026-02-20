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
use embassy_rp::pio::Pio;
use static_cell::StaticCell;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

static STATE: StaticCell<cyw43::State> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut lcd_led = Output::new(p.PIN_12, Level::Low);
    let dc = Output::new(p.PIN_13, Level::Low);
    let cs = Output::new(p.PIN_5, Level::High);
    let rst = Output::new(p.PIN_14, Level::Low);

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = cyw43_pio::PioSpi::new(&mut pio.common, pio.sm0, pio.DMA_CH0, cs, p.PIN_24, p.PIN_29);

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

    let fw = cyw43_firmware::CYW43_43439A0;
    let clm = cyw43_firmware::CYW43_43439A0_CLM;
    loop {}
}
