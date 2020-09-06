use crate::library::types::*;
use crate::library::shifter::*;
use crate::protocol;
use protocol::*;
use std::collections::HashMap;

pub struct SensorProcessing {
    pub motor_positions: HashMap<u8, i32>,
    pub shifters: Vec<Shifter>
}

impl SensorProcessing {
    pub fn new() -> SensorProcessing {
           SensorProcessing  { motor_positions: HashMap::new(), shifters: Vec::new()} 
    }
}

impl SensorProcessing {
    pub fn processing(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
        let position_updates = messenger.receive_message(&messages::MotorPositionUpdate::get_topic());
        let ret = position_updates.len() > 0;

        for p in position_updates {
            let meas = serde_json::from_str::<MotorPositionUpdate>(&p);

            match meas {
                Ok(v) => {
                    self.motor_positions.insert(v. port as u8, v.position);
                }
                Err(e) => log::error!("Error in JSON deserialization: {:?}", e)
            }
        }
        ret
    }
}