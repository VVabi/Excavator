use crate::state_machine_lib;
use crate::library::types::*;
use crate::protocol;
use crate::sensor_processing::sensor_processing_root::*;
use state_machine_lib::*;
use protocol::*;
use super::excavator_release::*;

#[derive(Debug)]
pub enum ExcavatorGripStates {
    Idle,
    OpenShovel,
    WaitForShovelOpened,
    LowerArm,
    WaitForLoweredArm,
    Grip,
    WaitForGrip,
    RaiseArm,
    WaitForRaisedArm,
}

pub struct ExcavatorGrip{
    pub state: ExcavatorGripStates,
}

impl ExcavatorGrip {
    pub fn new() -> ExcavatorGrip {
        ExcavatorGrip {state : ExcavatorGripStates::OpenShovel}
    }
}

impl StateMachine for ExcavatorGrip {
    fn check_abort_children(self: &mut Self, messenger: &mut dyn Messenger) -> bool {
         false
    }
    fn step(self: &mut Self, messenger: &mut dyn Messenger, sensor_proc: &mut SensorProcessing) -> StateMachineRetValue {
        let ret = StateMachineResult::Ongoing;
        let mut child: Option<Box<dyn StateMachine>> = None;
        match {&self.state} {
            ExcavatorGripStates::Idle => {

            }
            ExcavatorGripStates::OpenShovel => {
                sensor_proc.actuators.get_mut("shovel").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                self.state = ExcavatorGripStates::WaitForShovelOpened;
            }
            ExcavatorGripStates::WaitForShovelOpened => {
                if sensor_proc.actuators.get_mut("shovel").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) {
                    self.state = ExcavatorGripStates::LowerArm;
                }
            }
            ExcavatorGripStates::LowerArm => {
                sensor_proc.actuators.get_mut("lower_arm").unwrap().start_extend_actuator(messenger, 0.0).unwrap();
                sensor_proc.actuators.get_mut("higher_arm").unwrap().start_extend_actuator(messenger, 0.0).unwrap();
                self.state = ExcavatorGripStates::WaitForLoweredArm;
            }
            ExcavatorGripStates::WaitForLoweredArm => {
                if sensor_proc.actuators.get_mut("higher_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) &&  sensor_proc.actuators.get_mut("lower_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions){
                    self.state = ExcavatorGripStates::Grip;
                }
            }
            ExcavatorGripStates::Grip => {
                sensor_proc.actuators.get_mut("shovel").unwrap().start_extend_actuator(messenger, 0.0).unwrap();
                self.state = ExcavatorGripStates::WaitForGrip;
            }
            ExcavatorGripStates::WaitForGrip => {
                if sensor_proc.actuators.get_mut("shovel").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) {
                    self.state = ExcavatorGripStates::RaiseArm;
                }
            }
            ExcavatorGripStates::RaiseArm => {
                sensor_proc.actuators.get_mut("lower_arm").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                sensor_proc.actuators.get_mut("higher_arm").unwrap().start_extend_actuator(messenger, 1.0).unwrap();
                self.state = ExcavatorGripStates::WaitForRaisedArm;
            }
            ExcavatorGripStates::WaitForRaisedArm => {
                if sensor_proc.actuators.get_mut("higher_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions) &&  sensor_proc.actuators.get_mut("lower_arm").unwrap().check_extend_actuator_finished(&sensor_proc.motor_positions){
                    child = Some(Box::new(ExcavatorRelease::new()));
                    self.state = ExcavatorGripStates::Idle;
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