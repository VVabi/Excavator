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
use protocol::messages::*;

fn main() {
    env_logger::init();
    let subscriptions = vec![MotorPositionUpdate::get_topic()];
    let (tx, rx) = launch_mqtt("localhost".to_string(), 1883, subscriptions);
    let mut mqtt_messenger = MqttMessenger::new(&tx, &rx);

    let x = state_machines::top_level::TopLevelSm::new();

    let mut manager         = StateMachineManager { sm_stack: Vec::new()};
    let mut sensor_proc      = SensorProcessing::new();

    let mut shifter = library::shifter::Shifter {angle_diffs: vec![0, 180], port: Port::C, start_position: None};
    shifter.init_calibration(& mut mqtt_messenger, & mut sensor_proc);

    let two_seconds = std::time::Duration::from_millis(2000);
    std::thread::sleep(two_seconds);
    sensor_proc.processing(&mut mqtt_messenger);
    shifter.finish_calibration(& mut sensor_proc);

    shifter.shift(&mut mqtt_messenger, 1);
    sensor_proc.shifters.push(shifter);
    std::thread::sleep(two_seconds);

    let mut lower_act = library::actuator::Actuator {direction_sign: -1, gear_ratio: 1.25, length_in: 12.0, length_out: 17.0, port: Port::A, rotational_range: 9600.0, pulled_out_position: None, target_position: 0};

    lower_act.init_calibration(& mut mqtt_messenger, & mut sensor_proc);
    let forty_seconds = std::time::Duration::from_millis(40000);
    std::thread::sleep(forty_seconds);
    sensor_proc.processing(&mut mqtt_messenger);
    lower_act.finish_calibration(& mut sensor_proc);

    lower_act.start_extend_actuator(& mut mqtt_messenger, 0.5).unwrap();
    sensor_proc.processing(&mut mqtt_messenger);
    while !lower_act.check_extend_actuator_finished(& mut sensor_proc) {
        sensor_proc.processing(&mut mqtt_messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }
    std::thread::sleep(two_seconds);

    lower_act.start_extend_actuator(& mut mqtt_messenger, 1.0).unwrap();
    sensor_proc.processing(&mut mqtt_messenger);
    while !lower_act.check_extend_actuator_finished(& mut sensor_proc) {
        sensor_proc.processing(&mut mqtt_messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

    std::thread::sleep(two_seconds);

    lower_act.start_extend_actuator(& mut mqtt_messenger, 0.0).unwrap();
    sensor_proc.processing(&mut mqtt_messenger);
    while !lower_act.check_extend_actuator_finished(& mut sensor_proc) {
        sensor_proc.processing(&mut mqtt_messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

    std::thread::sleep(two_seconds);

    lower_act.start_extend_actuator(& mut mqtt_messenger, 0.3).unwrap();
    sensor_proc.processing(&mut mqtt_messenger);
    while !lower_act.check_extend_actuator_finished(& mut sensor_proc) {
        sensor_proc.processing(&mut mqtt_messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

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
