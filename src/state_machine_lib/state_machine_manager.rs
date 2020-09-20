use super::state_machine_definitions::*;
use std::error::Error;
use std::fmt;
use crate::library::types::*;
use crate::sensor_processing::sensor_processing_root::*;
use colored::*;

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
        log::info!("Starting to execute state machine {:?}", sm.get_name());
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
            self.sm_stack.pop(); //TODO add debug code
            //log::info!("State machine {:?} aborted", y.get_name());
        }

        let current_sm_res = self.sm_stack.pop();
        match {current_sm_res} {
            Some(mut current_sm) => {
                let prev_state  = current_sm.get_current_state();
                let ret         = current_sm.step(messenger, sensor_proc);
                let after_state = current_sm.get_current_state();

                if after_state != prev_state { //TODO this can actually be confusing because it is printed BEFORE a child is executed
                    let indent = 4*self.sm_stack.len();
                    log::info!("{:indent$}State transition from {} to {} in {}", "", prev_state.red(), after_state.red(), current_sm.get_name().green(), indent=indent)
                }

                if ret.result == StateMachineResult::Done {
                    let indent = 4*self.sm_stack.len();
                    log::info!("{:indent$}State machine {} finished", "", current_sm.get_name().green(), indent=indent);
                    let parent = self.sm_stack.pop();

                    if let Some(mut p) = parent {
                        let prev_state  = p.get_current_state();
                        p.set_child_result(&ret);
                        let after_state = p.get_current_state();
                        if after_state != prev_state { //TODO this can actually be confusing because it is printed BEFORE a child is executed
                            let indent = 4*self.sm_stack.len();
                            log::info!("{:indent$}State transition from {} to {} in {}", "", prev_state.red(), after_state.red(), p.get_name().green(), indent=indent)
                        }
                        self.sm_stack.push(p);
                    }

                    return Result::Ok(ret.result);
                }

                self.sm_stack.push(current_sm);

                if let Some(x) = ret.child {
                    let indent = 4*self.sm_stack.len();
                    log::info!("{:indent$}Starting to execute state machine {} in state {}", "", x.get_name().green(), x.get_current_state().red(), indent=indent);
                    self.sm_stack.push(x);
                }
                return Result::Ok(ret.result);
            },
            None => return Result::Err(Box::new(StateMachineError("Ran out of SMs!".to_string())))
        }
    }
}

