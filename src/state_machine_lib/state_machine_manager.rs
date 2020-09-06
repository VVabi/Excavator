use super::state_machine_definitions::*;
use std::error::Error;
use std::fmt;
use crate::library::types::*;
use crate::sensor_processing::sensor_processing_root::*;

pub struct StateMachineManager {
    pub sm_stack: Vec<Box<dyn StateMachine>>
}

#[derive(Debug)]
pub struct StateMachineError(String);

impl fmt::Display for StateMachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for StateMachineError {}

impl StateMachineManager {
    pub fn launch(self: &mut Self, sm: Box<dyn StateMachine>) -> Result<(), Box<StateMachineError>> {
        if !self.sm_stack.is_empty() {
            return  Result::Err(Box::new(StateMachineError("SM already running!".to_string())));
        }

        self.sm_stack.push(sm);
        Ok(())
    }

    pub fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> Result<StateMachineResult, Box<dyn Error>> {
        let mut num_to_kill = 0;
        let mut kill = false;
        for sm in &mut self.sm_stack {
            if kill {
                num_to_kill += 1;
            }
            else if sm.check_abort_children(messenger) {
                kill = true;
            }
        }

        for _x in 0..num_to_kill {
            self.sm_stack.pop();
        }

        let current_sm_res = self.sm_stack.pop();
        match {current_sm_res} {
            Some(mut current_sm) => {
                let ret = current_sm.step(messenger, sensor_proc);
                if ret.result == StateMachineResult::Done {
                    return Result::Ok(ret.result);
                }

                self.sm_stack.push(current_sm);

                if let Some(x) = ret.child {
                    println!("Pushing child!");
                    self.sm_stack.push(x);
                }
                return Result::Ok(ret.result);
            },
            None => return Result::Err(Box::new(StateMachineError("Ran out of SMs!".to_string())))
        }
    }
}

