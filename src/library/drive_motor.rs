use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use protocol::*;
use std::error::*;
use std::collections::HashMap;

pub struct DriveMotor {
    gear_ratio: f64,
    port: Port,
    wheel_radius: f64, //cm
    invert_direction: bool,
    target: i32,
    last_direction: i32
}

impl DriveMotor {
    pub fn new(gear_ratio: f64, wheel_radius: f64, invert_direction: bool, port: Port) -> DriveMotor {
        DriveMotor {gear_ratio: gear_ratio, wheel_radius: wheel_radius, invert_direction: invert_direction, port: port, target: 0, last_direction: 1}
    }

    pub fn check_change_direction(self: &mut Self, distance: f64) -> bool {
        let mut larger_zero = distance > 0.0;
        if self.invert_direction {
            larger_zero = !larger_zero;
        }

        if self.last_direction > 0 {
            return !larger_zero;
        } else {
            return larger_zero;
        }
    }


    pub fn start_moving(self: &mut Self, distance: f64, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) {
        let circumference = self.wheel_radius*2.0*std::f64::consts::PI;
        let rotations = distance/circumference*1.0/self.gear_ratio;
        let mut degrees = rotations*360.0;

        if self.invert_direction {
            degrees = -degrees;
        }

        let key         = self.port as u8;
        let start       = sensor_proc.motor_positions.get(&key).unwrap();
        let target      = start+degrees as i32;

        let goto_position = MotorGoToPosition { port: self.port, max_power: 100, pwm: 50, target_angle: target};
        if let Err(e) = messenger.publish_message(&goto_position) {
            log::error!("Error on publish: {:?}", e);
        }
        self.target = target;
        if degrees < 0.0 {
            self.last_direction = -1;
        } else {
            self.last_direction = 1;
        }
        log::info!("target {} current {}", target, start);
    }

    pub fn check_finished_driving(self: &Self, motor_positions: &HashMap<u8, i32>) -> bool {
        let key     = self.port as u8;
        let value   = motor_positions[&key];
        return (value-self.target).abs() < 30 || (self.last_direction == 1 && value > self.target) || (self.last_direction == -1 && value < self.target) //TODO remember the direction we came from
    }
}
