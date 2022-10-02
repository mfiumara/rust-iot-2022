use std::time::Duration;
use std::{str, sync::Arc, thread};

use embedded_svc::mqtt::client::{Client, Connection, Event, Message, Publish, QoS};
use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi};

use esp_idf_svc::{
    netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::EspWifi,
};

use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};

fn main() {
    // Initialize logging subsystem, let's us see logs from the esp-idf lower layers
    esp_idf_svc::log::EspLogger::initialize_default();

    // Initialize wifi configuration as a client configuration
    let mut wifi = EspWifi::new(
        Arc::new(EspNetifStack::new().unwrap()),
        Arc::new(EspSysLoopStack::new().unwrap()),
        Arc::new(EspDefaultNvs::new().unwrap()),
    )
    .unwrap();
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "<SSID>".into(),
        password: "<password>".into(),
        ..Default::default()
    }))
    .unwrap();

    // Give the wifi some time to initialize, could take a bit longer depending on the access point
    thread::sleep(Duration::from_secs(5));

    // Initialize the mqtt client, no password required since we are connecting to a public broker
    let mqtt_conf = MqttClientConfiguration {
        client_id: Some("my-esp32"),
        ..Default::default()
    };
    let (mut mqtt_client, mut connection) =
        EspMqttClient::new_with_conn("mqtt://broker.emqx.io", &mqtt_conf).unwrap();

    // Spawn thread to listen to incoming events on the mqtt connection
    thread::spawn(move || {
        println!("MQTT Listening for messages");

        while let Some(msg) = connection.next() {
            match msg {
                Err(e) => println!("MQTT Message ERROR: {}", e),
                Ok(event) => {
                    println!("MQTT Event: {:?}", event);
                    // Wait for a received event
                    match event {
                        Event::Received(rcv) => {
                            let s = match str::from_utf8(rcv.data()) {
                                Ok(v) => v,
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };
                            println!("rcv: {:?}", s);
                        }
                        // Ignore all other events for now
                        _ => {}
                    }
                }
            }
        }

        println!("MQTT connection loop exit");
    });

    mqtt_client
        .subscribe("my/test/sub", QoS::AtMostOnce)
        .unwrap();
    println!("Subscribed to my/test/sub");

    loop {
        thread::sleep(Duration::from_secs(1));
        mqtt_client
            .publish(
                "my/test/pub",
                QoS::AtMostOnce,
                false,
                "Hello world".as_bytes(),
            )
            .unwrap();
        println!("Publishing done!");
    }
}
