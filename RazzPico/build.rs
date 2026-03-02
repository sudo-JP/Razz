use std::env;
use dotenvy::dotenv;

fn split_ip(a: &str) -> Vec<String> {
    a.split('.')
        .map(str::to_string)
        .collect()
}

fn main() {
    dotenv().expect(".env file not found");

    let ssid = env::var("WIFI_SSID")
        .expect("Wifi SSID Not Set");

    let password = env::var("WIFI_PASSWORD")
        .expect("Wifi Password Not Set");

    println!("cargo:rustc-env=WIFI_SSID={}", ssid);
    println!("cargo:rustc-env=WIFI_PASSWORD={}", password);

    // IP Parsing
    let pico_addr = env::var("PICO_ADDR")
        .expect("Pico Addr not found");
    let pico_ips = split_ip(&pico_addr);

    pico_ips
        .iter()
        .enumerate()
        .for_each(|(index, value)| {
            println!("cargo:rustc-env=PICO_ADDR_{}={}", index, value);
        });

    let gateway = env::var("GATEWAY_IP")
        .expect("Gateway not found");
    let gateway_ips = split_ip(&gateway);

    gateway_ips 
        .iter()
        .enumerate()
        .for_each(|(index, value)| {
            println!("cargo:rustc-env=GATEWAY_{}={}", index, value);
        });

    let broker = env::var("BROKER")
        .expect("Broker not found");
    let broker_ips = split_ip(&broker);

    broker_ips 
        .iter()
        .enumerate()
        .for_each(|(index, value)| {
            println!("cargo:rustc-env=BROKER_{}={}", index, value);
        });

    // MQTT 
    let mqtt_port = env::var("MQTT_PORT")
        .expect("MQTT Port not found");
    println!("cargo:rustc-env=MQTT_PORT={}", mqtt_port);
         
    let mqtt_topic = env::var("MQTT_TOPIC")
        .expect("MQTT Topic Not Set");
    println!("cargo:rustc-env=MQTT_TOPIC={}", mqtt_topic);

}
