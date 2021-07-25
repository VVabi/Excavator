use crate::state_machine_lib;
use crate::library::types::*;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use std::collections::HashMap;
use std::fmt::*;
use std::fmt;

#[derive(Debug)]
pub enum MoveActuatorsState {
    Init,
    WaitForDone
}

impl Display for MoveActuatorsState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct MoveActuators{
    pub state:  MoveActuatorsState,
    pub targets: HashMap<String, f64>
}

impl MoveActuators {
    pub fn new(to_move: HashMap<String, f64>) -> MoveActuators {
        MoveActuators {state : MoveActuatorsState::Init, targets: to_move}
    }
}

impl StateMachine for MoveActuators {
    fn get_current_state(self: &Self) -> String {
        return self.state.to_string();
    }
    fn get_name(self: &Self) -> String {
        return "MoveActuators".to_string();
    }
    fn check_abort_children(self: &mut Self, _messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let mut ret = StateMachineResult::Ongoing;
        let child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            MoveActuatorsState::Init => {
                for (key, value) in &self.targets {
                    let actuator = sensor_proc.actuators.get_mut(key).unwrap();
                    actuator.start_extend_actuator(messenger, & mut sensor_proc.motor_feedback, *value).unwrap();
                }
                self.state = MoveActuatorsState::WaitForDone;
            }
            MoveActuatorsState::WaitForDone => {
                let mut done = true;
                for (key, _value) in &self.targets {
                    done = done && sensor_proc.actuators.get_mut(key).unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions, &sensor_proc.motor_feedback);
                }

                if done {
                    ret = StateMachineResult::Done;
                }
            }

        }

        return StateMachineRetValue {
            result: ret, child: child
        }
    }

    fn set_child_result(self: &mut Self, _ret: &StateMachineRetValue) {

    }
}


