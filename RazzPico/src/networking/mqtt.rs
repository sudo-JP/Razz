use embassy_net::{tcp::TcpSocket, IpAddress, IpEndpoint};
use rust_mqtt::{buffer::BumpBuffer, client::{options::ConnectOptions, Client}, types::MqttString};
use static_cell::StaticCell;
use rust_mqtt::config::{KeepAlive, SessionExpiryInterval};

use crate::networking::IPv4Addr;

pub struct MqttClient {
    broker: IPv4Addr,
    port: u16,
    topic: &'static str,
}

const BUFFER_SIZE: usize = 1024;
const MAX_SUBSCRIBES: usize = 1;
const RECEIVE_MAX: usize = 1; 
const SEND_MAX: usize = 1;

const TX_LEN: usize = 4096;
const RX_LEN: usize = 4096;

impl MqttClient {
    pub fn new() -> Self {
        let ip = IPv4Addr::get_broker();
        let port = 9000; // Change this soon
        let topic = "something burger"; // Change this too 

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
    }
}
