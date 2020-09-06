use crate::state_machine_lib;
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use protocol::*;

#[derive(Debug)]
pub enum TopLevelState {
    Idle,
}

pub struct TopLevelSm{
    pub state: TopLevelState,
}

impl TopLevelSm {
    pub fn new() -> TopLevelSm {
        TopLevelSm {state : TopLevelState::Idle}
    }
}

impl StateMachine for TopLevelSm {
    fn check_abort_children(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, messenger: &mut dyn Messenger, _sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            TopLevelState::Idle => {

            }
        }

        return StateMachineRetValue {
            result: ret, child: child
        }
    }

    fn set_child_result(self: &mut Self, _ret: StateMachineRetValue) {

    }
}



