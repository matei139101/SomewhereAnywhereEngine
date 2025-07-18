use crate::engine::components::command_bus::command_bus::CommandType;

pub trait Event {
    fn execute(&mut self) -> Option<CommandType>;
    fn get_status(&self) -> &EventStatus;
}

#[derive(PartialEq, Debug)]
pub enum EventStatus {
    Pending,
    Processing,
    Done,
    Failed,
}