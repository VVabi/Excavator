use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use protocol::*;
use std::error::*;

pub struct Actuator{
    pub rotational_range: f64,
    pub gear_ratio: f64, // actual rotation = motor_rotation*gear_ratio
    pub length_in: f64,
    pub length_out: f64,
    pub port: Port,
    pub direction_sign: i8, //positive: pulled in is lower position, negative: pulled in is higher position
    pub pulled_out_position: Option<i32>,
    pub target_position: i32
}


impl Actuator {
    pub fn init_calibration(self: &mut Self, messenger: &mut dyn Messenger, _sensor_proc: &mut SensorProcessing) {
        log::info!("Starting Actuator calibration");
        let enable_position_pdates = EnableModeUpdates {mode:2, port: self.port, notifications_enabled: 1, delta: 5 };
        if let Err(e) = messenger.publish_message(&enable_position_pdates) {
            log::error!("Error on publish: {:?}", e);
        }
        
        
        let angle = (self.rotational_range*1.0/self.gear_ratio*(self.direction_sign as f64)) as i32;
        let goto_position = MotorGoToPosition { port: self.port, max_power: 20, pwm: 30, target_angle: angle};
        if let Err(e) = messenger.publish_message(&goto_position) {
            log::error!("Error on publish: {:?}", e);
        }
    }

    pub fn finish_calibration(self: &mut Self, sensor_proc: &mut SensorProcessing) {
        let key = self.port as u8;
        let value = sensor_proc.motor_positions[&key];
        log::info!("calibrated start position of Actuator: {}", value);
        self.pulled_out_position = Some(value);
    }

    pub fn start_extend_actuator(self: &mut Self, messenger: &mut dyn Messenger, ratio: f64) -> Result<(), Box<dyn Error>> {
        if ratio < 0.0 || ratio > 1.0 {
            return Ok(()) //TODO!! return error
        }

        let rotational_motor_range = 1.0/self.gear_ratio*self.rotational_range;
        let rotational_ratio = rotational_motor_range*(1.0-ratio);
        self.target_position  = ((self.pulled_out_position.unwrap() as f64) - (self.direction_sign as f64)*rotational_ratio) as i32;
        log::info!("Actuator target position: {}", self.target_position);
        let goto_position = MotorGoToPosition { port: self.port, max_power: 70, pwm: 100, target_angle: self.target_position};
        if let Err(e) = messenger.publish_message(&goto_position) {
            log::error!("Error on publish: {:?}", e);
        }

        Ok(())
    }

    pub fn check_extend_actuator_finished(self: &mut Self, sensor_proc: &mut SensorProcessing) -> bool {
        let key = self.port as u8;
        let value = sensor_proc.motor_positions[&key];

        return (value-self.target_position).abs() < 100;
    }

}