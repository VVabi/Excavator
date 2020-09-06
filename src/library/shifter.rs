
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use protocol::*;


pub struct Shifter{
    pub angle_diffs : Vec<i32>,
    pub start_position: Option<i32>,
    pub port: Port
}

impl Shifter {
    pub fn init_calibration(self: &mut Self, messenger: &mut dyn Messenger, _sensor_proc: &mut SensorProcessing) {
        log::info!("Starting Shift calibration");
        let enable_position_updates = EnableModeUpdates {mode:2, port: self.port, notifications_enabled: 1, delta: 5 };
        if let Err(e) = messenger.publish_message(&enable_position_updates) {
            log::error!("Error on publish: {:?}", e);
        }
        
        let mut sign = -1;

        if self.angle_diffs.len() > 1 {
            let diff = self.angle_diffs[1]-self.angle_diffs[0];

            if diff < 0 {
                sign = 1;
            }
            log::info!("Sending shift motor to extreme position");
            let goto_position = MotorGoToPosition { port: self.port, max_power: 60, pwm: 50, target_angle: 10000*sign};
            if let Err(e) = messenger.publish_message(&goto_position) {
                log::error!("Error on publish: {:?}", e);
            }
        } 
    }

    pub fn finish_calibration(self: &mut Self, sensor_proc: &mut SensorProcessing) {
        let key = self.port as u8;
        let value = sensor_proc.motor_positions[&key];
        log::info!("calibrated start position of shifter: {}", value);
        self.start_position = Some(value);
    }

    pub fn shift(self: &mut Self, messenger: &mut dyn Messenger, gear: usize) {
        let angle = self.angle_diffs.get(gear);

        match angle {
            Some(x) => {
                let goto_position = MotorGoToPosition { port: self.port, max_power: 80, pwm: 100, target_angle: x+self.start_position.unwrap()};
                if let Err(e) = messenger.publish_message(&goto_position) {
                    log::error!("Error on publish: {:?}", e);
                }
            }
            None   => log::error!("Trying to shift to non-existant gear")
        }
    }
}






