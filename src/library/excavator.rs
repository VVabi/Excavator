use crate::library::types::*;
use crate::library::shifter::*;
use crate::library::actuator::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use protocol::*;


pub fn init_motor(messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing, port: Port) {
    sensor_proc.clear_motor_flags(port as u8);
    let enable_position_updates = EnableModeUpdates {mode:2, port: port, notifications_enabled: 1, delta: 5 };
    if let Err(e) = messenger.publish_message(&enable_position_updates) {
        log::error!("Error on publish: {:?}", e);
    }
    
    let read_pos = PortInformationRequest {port_id: port as u8};
    if let Err(e) = messenger.publish_message(&read_pos) {
        log::error!("Error on publish: {:?}", e);
    }
    let key = port as u8;

    loop {
        sensor_proc.processing(messenger);
        let value = sensor_proc.motor_positions.get(&key);

        if let Some(_x) = value {
            break;
        }
    }
}


pub fn init_excavator(messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) {
    init_motor(messenger, sensor_proc, Port::A);
    init_motor(messenger, sensor_proc, Port::B);
    init_motor(messenger, sensor_proc, Port::C);
    init_motor(messenger, sensor_proc, Port::D);
    /*let two_seconds = std::time::Duration::from_millis(2000);
    let mut shifter = Shifter {angle_diffs: vec![0, 180], port: Port::C, start_position: None};
    shifter.init_calibration(messenger, sensor_proc);

    std::thread::sleep(two_seconds);
    sensor_proc.processing(messenger);
    shifter.finish_calibration(sensor_proc);

    shifter.shift(messenger, 1);
    sensor_proc.shifters.push(shifter);
    std::thread::sleep(two_seconds);

    let mut lower_act = Actuator {direction_sign: 1, gear_ratio: 1.0, length_in: 12.0, length_out: 17.0, port: Port::D, rotational_range: 9400.0, pulled_out_position: None, target_position: 0};
    lower_act.init_calibration(messenger, sensor_proc);

    let mut higher_act = Actuator {direction_sign: -1, gear_ratio: 1.25, length_in: 12.0, length_out: 17.0, port: Port::A, rotational_range: 9400.0, pulled_out_position: None, target_position: 0};
    higher_act.init_calibration(messenger, sensor_proc);

    let mut shovel_act = Actuator {direction_sign: -1, gear_ratio: 1.25, length_in: 5.0, length_out: 8.0, port: Port::B, rotational_range: 6800.0, pulled_out_position: None, target_position: 0};
    shovel_act.init_calibration(messenger, sensor_proc);

    while !lower_act.finish_calibration(sensor_proc)  {
        sensor_proc.processing(messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

    while !shovel_act.finish_calibration(sensor_proc)  {
        sensor_proc.processing(messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }


    while !higher_act.finish_calibration(sensor_proc)  {
        sensor_proc.processing(messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

    while !shovel_act.finish_calibration(sensor_proc)  {
        sensor_proc.processing(messenger);

        let hundred_millis = std::time::Duration::from_millis(100);
        std::thread::sleep(hundred_millis);
    }

    sensor_proc.actuators.insert("lower_arm".to_string(), lower_act);
    sensor_proc.actuators.insert("higher_arm".to_string(), higher_act);
    sensor_proc.actuators.insert("shovel".to_string(), shovel_act);

    sensor_proc.shifters.get_mut(0).unwrap().shift(messenger, 0);*/
}