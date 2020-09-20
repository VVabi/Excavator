use crate::library::types::*;
use crate::sensor_processing::sensor_processing_root::*;

#[derive(Debug, PartialEq)]
pub enum StateMachineResult {
    Done,
    Ongoing
}



pub trait StateMachine {
    fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> StateMachineRetValue;
    fn set_child_result(self: &mut Self, ret: StateMachineRetValue);
    fn check_abort_children(self: &mut Self, messenger: &mut dyn Messenger) -> bool;
    fn get_name(self: &Self) -> String;
    fn get_current_state(self: &Self) -> String;
}

pub struct StateMachineRetValue {
    pub result: StateMachineResult,
    pub child: Option<Box<dyn StateMachine>>,
}


