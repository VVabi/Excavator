use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use protocol::*;
use std::error::*;
use std::collections::HashMap;

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
    pub fn init_calibration(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) {
        log::debug!("Starting Actuator calibration");
        let key = self.port as u8;
        let start = sensor_proc.motor_positions.get(&key).unwrap();
        let angle = 2*(self.rotational_range*1.0/self.gear_ratio*(self.direction_sign as f64)) as i32;

        let goto_position = MotorGoToPosition { port: self.port, max_power: 20, pwm: 100, target_angle: start+angle};
        if let Err(e) = messenger.publish_message(&goto_position) {
            log::error!("Error on publish: {:?}", e);
        }
    }

    pub fn finish_calibration(self: &mut Self, sensor_proc: &mut SensorProcessing) -> bool {
        let key = self.port as u8;
        if !sensor_proc.is_motor_cmd_discarded(key) {
            log::debug!("Actuator calibration not done: cmd not discarded yet");
            return false;
        }

        let value = sensor_proc.motor_positions.get(&key);

        if let Some(x) = value {

            let offset;
            if self.direction_sign > 0 {
                offset = -100;
            } else {
                offset = 100;
            }

            let mut y: i32 = *x;

            y = y+offset;
            log::debug!("calibrated start position of Actuator: {:?}", y);
            self.pulled_out_position = Some(y);
            return true
        } else {
            log::debug!("Actuator calibration not done: missing motor position");
        }
        false
    }

    pub fn start_extend_actuator(self: &mut Self, messenger: &mut dyn Messenger, motor_feedback: & mut HashMap<u8, u8>, ratio: f64) -> Result<(), Box<dyn Error>> {
        if ratio < 0.0 || ratio > 1.0 {
            return Ok(()) //TODO!! return error
        }

        let rotational_motor_range = 1.0/self.gear_ratio*self.rotational_range;
        let rotational_ratio = rotational_motor_range*(1.0-ratio);
        self.target_position  = ((self.pulled_out_position.unwrap() as f64) - (self.direction_sign as f64)*rotational_ratio) as i32;
        log::debug!("Actuator target position: {}", self.target_position);
        motor_feedback.remove_entry(&(self.port as u8));
        let goto_position = MotorGoToPosition { port: self.port, max_power: 70, pwm: 100, target_angle: self.target_position};
        if let Err(e) = messenger.publish_message(&goto_position) {
            log::error!("Error on publish: {:?}", e);
        }

        Ok(())
    }

    pub fn check_extend_actuator_finished(self: &mut Self, motor_positions: &HashMap<u8, i32>, motor_feedback: & HashMap<u8, u8>) -> bool {
        //TODO add a timeout
        let key = self.port as u8;
        let value = motor_positions[&key];

        let m = motor_feedback.get(&(self.port as u8));
        let mut motor_cmd_discarded = false;
        if let Some(x) = m {
            motor_cmd_discarded = (x & 4) != 0
        }

        return ((value-self.target_position).abs() < 100) || motor_cmd_discarded;
    }

}