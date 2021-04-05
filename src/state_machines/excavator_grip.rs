use crate::state_machine_lib;
use crate::library::types::*;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use super::move_actuators::*;
use std::collections::HashMap;
use std::fmt::*;
use std::fmt;

#[derive(Debug)]
pub enum ExcavatorGripStates {
    Idle,
    OpenShovel,
    LowerArm,
    Grip,
    RaiseArm,
}

impl Display for ExcavatorGripStates {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct ExcavatorGrip{
    pub state: ExcavatorGripStates,
    child_done_cb: Option<Box<dyn Fn(& mut ExcavatorGripStates)>>
}

impl ExcavatorGrip {
    pub fn new() -> ExcavatorGrip {
        ExcavatorGrip {state : ExcavatorGripStates::OpenShovel, child_done_cb: None}
    }
}

impl StateMachine for ExcavatorGrip {
    fn get_current_state(self: &Self) -> String {
        return self.state.to_string();
    }
    fn get_name(self: &Self) -> String {
        return "ExcavatorGrip".to_string();
    }
    fn check_abort_children(self: &mut Self, _messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let mut ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            ExcavatorGripStates::Idle => {
                ret = StateMachineResult::Done;
            }
            ExcavatorGripStates::OpenShovel => {
                sensor_proc.shifters[0].shift(messenger, 1);
                let mut targets = HashMap::new();
                targets.insert("shovel".to_string(), 1.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorGripStates| *x = ExcavatorGripStates::LowerArm));
            }

            ExcavatorGripStates::LowerArm => {
                let mut targets = HashMap::new();
                targets.insert("lower_arm".to_string(), 0.0);
                targets.insert("higher_arm".to_string(), 0.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorGripStates| *x = ExcavatorGripStates::Grip));
            }

            ExcavatorGripStates::Grip => {
                let mut targets = HashMap::new();
                targets.insert("shovel".to_string(), 0.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorGripStates| *x = ExcavatorGripStates::RaiseArm));
            }
            ExcavatorGripStates::RaiseArm => {
                let mut targets = HashMap::new();
                targets.insert("lower_arm".to_string(), 1.0);
                targets.insert("higher_arm".to_string(), 1.0);
                child = Some(Box::new(MoveActuators::new(targets)));
                self.child_done_cb = Some(Box::new(|x: &mut ExcavatorGripStates| *x = ExcavatorGripStates::Idle));
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