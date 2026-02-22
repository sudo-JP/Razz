use cyw43::JoinOptions;
use embassy_executor::Spawner;
use embassy_rp::{gpio::{Level, Output}};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_rp::{bind_interrupts};
use embassy_net::{StackResources};
use embassy_rp::pio::{InterruptHandler, Pio};
use static_cell::StaticCell;
use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_23, PIN_24, PIN_25, PIN_29, DMA_CH0, PIO0};
use embassy_rp::clocks::RoscRng;

use embassy_time::{Timer, Duration};
use embassy_futures::select::{select, Either};

pub struct WifiPins<'d> {
    pub pwr: Peri<'d, PIN_23>,
    pub cs: Peri<'d, PIN_25>,
    pub dio: Peri<'d, PIN_24>,
    pub clk: Peri<'d, PIN_29>,
    pub dma: Peri<'d, DMA_CH0>,
    pub pio: Peri<'d, PIO0>,
}

pub struct Wifi {
    control: cyw43::Control<'static>,
}

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

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, 
    cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

impl Wifi {
    pub async fn new(spawner: Spawner, p: WifiPins<'static>) 
        -> (Self, &'static embassy_net::Stack<'static>) {
        let cs = Output::new(p.cs, Level::High);
        let pwr = Output::new(p.pwr, Level::Low);
        let mut pio = Pio::new(p.pio, Irqs);

        // Network
        let spi = cyw43_pio::PioSpi::new(
            &mut pio.common, 
            pio.sm0, 
            DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            cs, 
            p.dio, 
            p.clk,
            p.dma
        );

        let fw = cyw43_firmware::CYW43_43439A0;
        let clm = cyw43_firmware::CYW43_43439A0_CLM;

        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw)
            .await;

        spawner.spawn(wifi_task(runner)).unwrap();

        control.init(clm).await;
        /*control
            .set_power_management(cyw43::PowerManagementMode::PowerSave)
            .await;*/


        // stack setup
        let config = embassy_net::Config::dhcpv4(Default::default());
        let mut rng = RoscRng;

        static STACK: StaticCell<embassy_net::Stack<'static>> = StaticCell::new();
        static RESOURCE: StaticCell<StackResources<3>> = StaticCell::new();
        let (stack, runner) = embassy_net::new(
            net_device, 
            config, 
            RESOURCE.init(StackResources::new()), 
            rng.next_u64()
        );

        let stack = STACK.init(stack);
        spawner.spawn(net_task(runner)).unwrap();

        (Self { control }, stack)
    }

    pub async fn connect(&mut self) -> Result<(), &'static str> {
        match select(
            self.control.join(WIFI_SSID, JoinOptions::new(WIFI_PASSWORD.as_bytes())),
            Timer::after(Duration::from_secs(10))
        ).await {
            Either::First(Ok(_)) => Ok(()),
            Either::First(Err(_)) => Err("join failed"),
            Either::Second(_) => Err("timeout"),
        }
    }
}

