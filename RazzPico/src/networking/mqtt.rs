use embassy_net::{tcp::TcpSocket, IpAddress, IpEndpoint};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use rust_mqtt::{buffer::BumpBuffer, client::{event::{Event, Suback}, options::{ConnectOptions, RetainHandling, SubscriptionOptions}, Client}, types::{MqttString, QoS, TopicFilter, TopicName}};
use static_cell::StaticCell;
use rust_mqtt::config::{KeepAlive, SessionExpiryInterval};

use crate::networking::IPv4Addr;

pub struct MqttClient {
    broker: IPv4Addr,
    port: u16,
    topic: &'static str,
}

const BUFFER_SIZE: usize = 2048;
const MAX_SUBSCRIBES: usize = 1;
const RECEIVE_MAX: usize = 1; 
const SEND_MAX: usize = 1;

const TX_LEN: usize = 1024;
const RX_LEN: usize = 1024;
const MQTT_TOPIC: &str = env!("MQTT_TOPIC");

pub static MQTT_CHANNEL: Channel<CriticalSectionRawMutex, ([u8; 512], usize), 4> = Channel::new();

impl MqttClient {
    pub fn new() -> Self {
        let ip = IPv4Addr::get_broker();
        let port: u16 = env!("MQTT_PORT").parse().unwrap();
        let topic = MQTT_TOPIC;

        Self {
            broker: ip, 
            port: port, 
            topic: topic
        }
    }

    pub async fn run(&self, stack: &'static embassy_net::Stack<'static>) {
        let mut buffer = [0; BUFFER_SIZE];
        let mut buffer = BumpBuffer::new(&mut buffer);

        let mut client = Client::<'_, _, _, MAX_SUBSCRIBES, RECEIVE_MAX, SEND_MAX>::new(&mut buffer);

        static RX: StaticCell<[u8; RX_LEN]> = StaticCell::new();
        static TX: StaticCell<[u8; TX_LEN]> = StaticCell::new();
        let mut socket = TcpSocket::new(*stack, RX.init([0u8; RX_LEN]), TX.init([0u8; TX_LEN]));

        let ipv4 = IpAddress::Ipv4(self.broker.to_embassy_ip());
        socket.connect(IpEndpoint::new(
                ipv4, 
                self.port))
            .await.unwrap();


        let connect_opt = ConnectOptions{
            session_expiry_interval: SessionExpiryInterval::Seconds(60),
            clean_start: true, 
            keep_alive: KeepAlive::Seconds(30),
            will: None, 
            user_name: None,
            password: None,
        };

        client.connect(
            socket, 
            &connect_opt, 
            Some(MqttString::try_from("pico").unwrap()))
            .await
            .unwrap();

        let sub_opt = SubscriptionOptions {
            retain_handling: RetainHandling::SendIfNotSubscribedBefore,
            retain_as_published: true,
            no_local: false,
            qos: QoS::AtMostOnce,
        };

        let topic = unsafe {
            TopicName::new_unchecked(MqttString::from_slice(self.topic).unwrap()) 
        };

        client.subscribe(topic.clone().into(), sub_opt)
            .await
            .unwrap();
            
        // Handshake
        match client.poll().await {
            Ok(_) => {}
            Err(_) => { return; }
        }

        loop {
            match client.poll().await {
                Ok(Event::Publish(publish)) => {
                    let bytes: &[u8] = &publish.message;

                    let mut buf = [0u8; 512];
                    let len = bytes.len().min(512);
                    buf[..len].copy_from_slice(&bytes[..len]);
                    MQTT_CHANNEL.sender().send((buf, len)).await;
                }
                Ok(_) => {},
                Err(_) => {}
            }
        }
        
    }
}
