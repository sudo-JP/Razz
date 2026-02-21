#![no_std]
#![no_main]

use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};

// Hardware blah blah
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts};
use embassy_rp::gpio::{Level, Output};
use panic_halt as _;

use static_cell::StaticCell;

// Network
use embassy_net::tcp::TcpSocket;
use embassy_net::{StackResources};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::peripherals::{DMA_CH0, PIO0};

bind_interrupts!(struct Irqs{
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});
const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

#[embassy_executor::task]
async fn wifi_task(runner: cyw43::Runner<'static, Output<'static>, 
    PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let cs = Output::new(p.PIN_25, Level::High);
    let pwr = Output::new(p.PIN_23, Level::Low);
    let mut pio = Pio::new(p.PIO0, Irqs);

    // Network
    let spi = cyw43_pio::PioSpi::new(
        &mut pio.common, 
        pio.sm0, 
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs, 
        p.PIN_24, 
        p.PIN_29,
        p.DMA_CH0
    );

    let fw = cyw43_firmware::CYW43_43439A0;
    let clm = cyw43_firmware::CYW43_43439A0_CLM;

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw)
        .await;

    let _ = spawner.spawn(wifi_task(runner));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // LED

    loop {}
}
