extern crate serde_json;
extern crate serde;

mod protocol;
mod mqtt_wrapper;
mod library;
mod state_machine_lib;
mod sensor_processing;
mod state_machines;

use mqtt_wrapper::mqtt_thread::launch_mqtt;
use mqtt_wrapper::mqtt_messenger::MqttMessenger;
use state_machine_lib::state_machine_manager::*;
use sensor_processing::sensor_processing_root::*;

fn main() {
    env_logger::init();
    let subscriptions = vec![];
    let (tx, rx) = launch_mqtt("localhost".to_string(), 1883, subscriptions);
    let mut mqtt_messenger = MqttMessenger::new(&tx, &rx);

    let x = state_machines::top_level::TopLevelSm::new();

    let mut manager         = StateMachineManager { sm_stack: Vec::new()};
    let mut sensor_proc      = SensorProcessing::new();
    manager.launch(Box::new(x)).expect("Error during state machine manager launch");

    loop {
        let update_received = sensor_proc.processing(&mut mqtt_messenger);

        if update_received {
            let x = manager.step(&mut mqtt_messenger, &mut sensor_proc);

            match { x } {
                Ok(_m) => (),
                Err(_x) => break,
            }
        }
        let ten_millis = std::time::Duration::from_millis(10);
        std::thread::sleep(ten_millis);
    }
}
