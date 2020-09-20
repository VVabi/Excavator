use crate::state_machine_lib;
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use protocol::*;

#[derive(Debug)]
pub enum ExcavatorReleaseState {
    Idle,
    LowerArm,
    WaitForLoweredArm,
    OpenShovel,
    WaitForShovelOpened,
    RaiseArm,
    WaitForRaisedArm,
}

pub struct ExcavatorRelease{
    pub state: ExcavatorReleaseState,
}

impl ExcavatorRelease {
    pub fn new() -> ExcavatorRelease {
        ExcavatorRelease {state : ExcavatorReleaseState::LowerArm }
    }
}

impl StateMachine for ExcavatorRelease {
    fn check_abort_children(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            ExcavatorReleaseState::Idle => {

            }

            ExcavatorReleaseState::LowerArm => {
                sensor_proc.actuators.get_mut("lower_arm").unwrap().start_extend_actuator(messenger, 0.0).unwrap();
                sensor_proc.actuators.get_mut("higher_arm").unwrap().start_extend_actuator(messenger, 0.0).unwrap();
                self.state = ExcavatorReleaseState::WaitForLoweredArm;
            }
            ExcavatorReleaseState::WaitForLoweredArm => {
                if sensor_proc.actuators.get_mut("higher_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) &&  sensor_proc.actuators.get_mut("lower_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions){
                    self.state = ExcavatorReleaseState::OpenShovel;
                }
            }

            ExcavatorReleaseState::OpenShovel => {
                sensor_proc.actuators.get_mut("shovel").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                self.state = ExcavatorReleaseState::WaitForShovelOpened;
            }
            ExcavatorReleaseState::WaitForShovelOpened => {
                if sensor_proc.actuators.get_mut("shovel").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) {
                    self.state = ExcavatorReleaseState::RaiseArm;
                }
            }

            ExcavatorReleaseState::RaiseArm => {
                sensor_proc.actuators.get_mut("lower_arm").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                sensor_proc.actuators.get_mut("higher_arm").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                self.state = ExcavatorReleaseState::WaitForRaisedArm;
            }
            ExcavatorReleaseState::WaitForRaisedArm => {
                if sensor_proc.actuators.get_mut("higher_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) &&  sensor_proc.actuators.get_mut("lower_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions){
                    self.state = ExcavatorReleaseState::Idle;
                }
            }
        }

        return StateMachineRetValue {
            result: ret, child: child
        }
    }

    fn set_child_result(self: &mut Self, _ret: StateMachineRetValue) {

    }
}