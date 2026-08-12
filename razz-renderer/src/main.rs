//use std::time::Duration;
use clap::Parser;
use razz_renderer::{cli::Cli, generated};
/*use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};

use std::env;
use dotenvy::dotenv;*/


#[tokio::main]
async fn main()  {
    //let cli = Cli::parse();
    //cli.run();
    /*let mqttoptions = MqttOptions::new("rumqtt-async", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("razz/pico", QoS::AtMostOnce)
        .await
        .unwrap();
    task::spawn(async move {
        for i in 0..10u8 {
            client.publish("razz/pico", QoS::AtLeastOnce, false, format!("{}", i).as_bytes())
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });
    loop {
        let noti = eventloop
            .poll()
            .await 
            .unwrap();
        println!("Received = {:?}", noti);
    }*/
    generated::foo();
}

