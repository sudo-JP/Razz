#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use mipidsi::models::ST7735s;
use panic_halt as _;
use rp2040_hal::{self as hal, Clock};
use hal::pac;
use fugit::RateExtU32;

// Display
use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, TestImage};
use embedded_hal_bus::spi::ExclusiveDevice;

use embedded_graphics::{prelude::*, pixelcolor::Rgb565};

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;


/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    // Singleton
    let mut pac = pac::Peripherals::take().unwrap();

    // For clocks
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );


    // SPI driver 
    let _spi_sck = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
    let _spi_sda = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();


    let mut lcd_led = pins.gpio12.into_push_pull_output();
    let dc = pins.gpio13.into_push_pull_output();
    let rst = pins.gpio14.into_push_pull_output();
    let cs = pins.gpio5.into_push_pull_output();

    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI0, (_spi_sda, _spi_sck));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16_000_000u32.Hz(),
        embedded_hal::spi::MODE_0,
    );

    let spi = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut buffer = [0u8; 512];
    let di = SpiInterface::new(spi, dc, &mut buffer);

    let mut display = Builder::new(ST7735s, di)
        .reset_pin(rst)
        .init(&mut timer)
        .unwrap();

    display.clear(Rgb565::BLUE).unwrap();
    //TestImage::new().draw(&mut display);
    lcd_led.set_high().unwrap();
    loop {}

}
