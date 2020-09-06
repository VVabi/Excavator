use crate::library::types::*;
use crate::protocol;
use protocol::*;

pub struct BumperState {
    pub left: i32,
    pub right: i32,
    pub timestamp: i32
}

impl BumperState {
    pub fn new() -> BumperState {
           BumperState  { left: 0, right: 0, timestamp: 0} 
    }
}

pub struct SensorProcessing {
    pub bumper: BumperState
}

impl SensorProcessing {
    pub fn new() -> SensorProcessing {
           SensorProcessing  { bumper: BumperState::new()} 
    }
}

impl SensorProcessing {
    pub fn processing(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
        true
    }
}