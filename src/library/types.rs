use std::error::Error;
use crate::protocol::messages::Message;

pub trait Messenger {
    fn publish_message(self: &Self, m: &dyn Message) -> Result<(), Box<dyn Error>>;
    fn receive_message(self: &mut Self, topic: &str) -> Vec<std::string::String>;
}