use std::env;
use dotenvy::dotenv;

fn main() {
    dotenv().expect(".env file not found");

    let ssid = env::var("WIFI_SSID")
        .expect("Wifi SSID Not Set");

    let password = env::var("WIFI_PASSWORD")
        .expect("Wifi Password Not Set");

    println!("cargo:rustc-env=WIFI_SSID={}", ssid);
    println!("cargo:rustc-env=WIFI_PASSWORD={}", password);
}
