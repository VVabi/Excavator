use crate::state_machine_lib;
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use protocol::*;
use super::excavator_grip::*;
use super::excavator_release::*;
use super::movesm::*;
use super::place_excavator::*;
use std::fmt::*;
use std::fmt;
use std::{thread, time};

#[derive(Debug)]
pub enum TopLevelState {
    Idle,
    Grip,
    Release,
    Done
}

impl Display for TopLevelState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt::Debug::fmt(self, f)
    }
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
    fn get_current_state(self: &Self) -> String {
        return self.state.to_string();
    }
    fn get_name(self: &Self) -> String {
        return "TopLevel".to_string();
    }
    fn check_abort_children(self: &mut Self, _messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, _messenger: &mut dyn Messenger, _sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let mut ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            TopLevelState::Idle => {
                child = Some(Box::new(ExcavatorPlacementSM::new()));
                self.state = TopLevelState::Grip;
                let one_second = time::Duration::from_millis(1000); //HACK
                thread::sleep(one_second);
            }
            TopLevelState::Grip => {
                child = Some(Box::new(ExcavatorGrip::new()));
                self.state = TopLevelState::Release;
            }
            TopLevelState::Release => {
               child = Some(Box::new(ExcavatorRelease::new()));
               self.state = TopLevelState::Done;
            }
            TopLevelState::Done => {
                ret = StateMachineResult::Done;
            }
        }

        return StateMachineRetValue {
            result: ret, child: child
        }
    }

    fn set_child_result(self: &mut Self, _ret: &StateMachineRetValue) {

    }
}



