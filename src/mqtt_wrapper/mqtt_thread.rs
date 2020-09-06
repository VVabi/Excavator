extern crate paho_mqtt as mqtt;
use std::process;

use std::thread::spawn;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub struct MqttStrMessage {
    pub topic: std::string::String,
    pub payload: std::string::String
}



fn launch_mqtt_thread(tx: Sender<MqttStrMessage>, rx: Receiver<MqttStrMessage>, host: String, port: u32, subscriptions: Vec<String>) {
    let mut cli = mqtt::AsyncClient::new("tcp://".to_string()+&host+":"+&port.to_string()).unwrap_or_else(|err| {
        log::error!("Cannot create mqtt client: {}", err);
        process::exit(1);
    });
    let conn_opts = mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts).wait() {
        log::error!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    for topic in subscriptions {
        cli.subscribe(topic, 0);
    }
    cli.set_message_callback(move |_cli,msg| {
        if let Some(msg) = msg {
            let topic = msg.topic();
            let payload_str = msg.payload_str();
            let res = tx.send(MqttStrMessage {topic: String::from(topic), payload: String::from(payload_str) });
            if !res.is_ok() {
                log::error!("Error during mqtt receive");
            }
        }
    });

    loop {
        let msg = rx.recv();
        match msg {
            Ok(m) => {
                let mqtt_msg = mqtt::Message::new(m.topic, m.payload, 0);
                cli.publish(mqtt_msg);
            },
            Err(e) => log::error!("Mqtt publish channel: {:?}", e),
        }
    }
}


pub fn launch_mqtt(host: String, port: u32, subscriptions: Vec<String>) -> (Sender<MqttStrMessage>, Receiver<MqttStrMessage>) {

    let (tx, rx) = channel();
    let (tx_publish, rx_publish) = channel();
    spawn(move || launch_mqtt_thread(tx, rx_publish, host, port, subscriptions)); //TODO we should wait after this until mqtt connection is established

    (tx_publish, rx)
}