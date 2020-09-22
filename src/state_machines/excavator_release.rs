use crate::state_machine_lib;
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use std::collections::HashMap;
use super::move_actuators::*;
use std::fmt::*;
use std::fmt;

#[derive(Debug)]
pub enum ExcavatorReleaseState {
    Idle,
    LowerArm,
    OpenShovel,
    RaiseArm,
}

impl Display for ExcavatorReleaseState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct ExcavatorRelease{
    pub state: ExcavatorReleaseState,
    child_done_cb: Option<Box<dyn Fn(& mut ExcavatorReleaseState)>>
}

impl ExcavatorRelease {
    pub fn new() -> ExcavatorRelease {
        ExcavatorRelease {state : ExcavatorReleaseState::LowerArm, child_done_cb: None}
    }
}

impl StateMachine for ExcavatorRelease {
    fn get_current_state(self: &Self) -> String {
        return self.state.to_string();
    }
    fn get_name(self: &Self) -> String {
        return "ExcavatorRelease".to_string();
    }
    fn check_abort_children(self: &mut Self, _messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, _messenger: &mut dyn Messenger, _sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let mut ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            ExcavatorReleaseState::Idle => {
                ret = StateMachineResult::Done;
            }

            ExcavatorReleaseState::LowerArm => {
                let mut targets = HashMap::new();
                targets.insert("lower_arm".to_string(), 0.0);
                targets.insert("higher_arm".to_string(), 0.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorReleaseState| *x = ExcavatorReleaseState::OpenShovel));
            }

            ExcavatorReleaseState::OpenShovel => {
                let mut targets = HashMap::new();
                targets.insert("shovel".to_string(), 1.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorReleaseState| *x = ExcavatorReleaseState::RaiseArm));
            }

            ExcavatorReleaseState::RaiseArm => {
                let mut targets = HashMap::new();
                targets.insert("lower_arm".to_string(), 1.0);
                targets.insert("higher_arm".to_string(), 1.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorReleaseState| *x = ExcavatorReleaseState::Idle));
            }

        }

        return StateMachineRetValue {
            result: ret, child: child
        }
    }

    fn set_child_result(self: &mut Self, _ret: &StateMachineRetValue) {
        if let Some(x) = &self.child_done_cb {
            x(& mut self.state);
            self.child_done_cb = None;
        }
    }
}