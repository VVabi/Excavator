use crate::library::types::*;
use crate::library::shifter::*;
use crate::library::actuator::*;
use crate::library::drive_motor::*;
use crate::protocol;
use protocol::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize}; //HACK

pub struct SensorProcessing {
    pub motor_positions: HashMap<u8, i32>,
    pub motor_feedback: HashMap<u8, u8>,
    pub shifters: Vec<Shifter>,
    pub actuators: HashMap<String, Actuator>,
    pub drive_motors: HashMap<String, DriveMotor>,
    pub blob_location: Vec<u32>,
}

impl SensorProcessing {
    pub fn new() -> SensorProcessing {
           SensorProcessing  { motor_positions: HashMap::new(), motor_feedback: HashMap::new(), shifters: Vec::new(), actuators: HashMap::new(), drive_motors: HashMap::new(), blob_location: vec![0, 0]} 
    }
}

#[derive(Serialize, Deserialize)] //HACK
pub struct BlobLocation {
    pub x: u32,
    pub y: u32
}


impl SensorProcessing {
    pub fn processing(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
        let position_updates = messenger.receive_message(&messages::MotorPositionUpdate::get_topic());
        let mut ret = position_updates.len() > 0;

        for p in position_updates {
            let meas = serde_json::from_str::<MotorPositionUpdate>(&p);

            match meas {
                Ok(v) => {
                    self.motor_positions.insert(v.port as u8, v.position);
                }
                Err(e) => log::error!("Error in JSON deserialization: {:?}", e)
            }
        }

        let cmd_updates = messenger.receive_message(&messages::MotorCommandFeedback::get_topic());

        ret = ret || (cmd_updates.len() > 0);

        for p in cmd_updates {
            let meas = serde_json::from_str::<MotorCommandFeedback>(&p);

            match meas {
                Ok(v) => {
                    self.motor_feedback.insert(v.port as u8, v.flags);
                }
                Err(e) => log::error!("Error in JSON deserialization: {:?}", e)
            }
        }

        let blob_updates = messenger.receive_message(&"camera/blob".to_string());
        ret = ret || (blob_updates.len() > 0);
        for b in blob_updates {
            let meas = serde_json::from_str::<BlobLocation>(&b);

            match meas {
                Ok(v) => {
                    self.blob_location[0] = v.x;
                    self.blob_location[1] = v.y;
                }
                Err(e) => log::error!("Error in JSON deserialization: {:?}", e)
            }
        }

        ret
    }

    pub fn clear_motor_flags(self: &mut Self, port: u8) {
        self.motor_feedback.remove_entry(&port);
    }

    pub fn is_motor_cmd_discarded(self: &Self, port: u8) -> bool {
        let m = self.motor_feedback.get(&port);

        if let Some(x) = m {
            return (x & 4) != 0
        }

        false
    }
}