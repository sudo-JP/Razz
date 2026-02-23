#![no_std]
#![no_main]

use cortex_m::asm::delay;
// Hardware blah blah
use embassy_executor::Spawner;
use panic_halt as _;

use razzpico::networking::wifi::WifiPins;
use razzpico::{Renderer, RendererPins, Wifi};

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let rend_pins = RendererPins{
        pin_led: p.PIN_12.into(),
        pin_dc: p.PIN_13.into(),
        pin_cs: p.PIN_5.into(),
        pin_rst: p.PIN_14.into(),
        pin_sck: p.PIN_6.into(),
        pin_mosi: p.PIN_7.into(),
        pin_miso: p.PIN_4.into(),
        spi0: p.SPI0.into(),
    };
    let mut renderer = Renderer::new(rend_pins);
    renderer.on();
    renderer.clear();

    let wifi_pins = WifiPins{
        pwr: p.PIN_23.into(),
        cs: p.PIN_25.into(),
        dio: p.PIN_24.into(),
        clk: p.PIN_29.into(),
        dma: p.DMA_CH0.into(),
        pio: p.PIO0.into(),
    };

    renderer.text(WIFI_SSID, 1);
    renderer.text(WIFI_PASSWORD, 2);
    
    let (mut wifi, stack) = Wifi::new(spawner, wifi_pins)
        .await;
    renderer.text("Init Wifi", 3);
    stack.wait_link_up().await;
    renderer.text("Wait Link Up", 4);

    stack.wait_config_up().await;
    renderer.text("Wait Config Up", 5);

    loop {}
}
