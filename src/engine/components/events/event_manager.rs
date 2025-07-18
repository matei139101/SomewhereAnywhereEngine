use crate::engine::components::command_bus::command_bus::CommandType;
use crate::engine::components::events::subcomponents::event::{Event, EventStatus};
use crate::engine::utils::logger::{LogLevel, Logger};

pub struct EventManager {
    events: Vec<Box<dyn Event>>
}

impl EventManager {
    pub fn new() -> Self {
        return EventManager {
            events: vec![],
        }
    }

    pub fn process(&mut self) -> Vec<CommandType>{
        let mut commands: Vec<CommandType> = vec![];
        
        for event in self.events.iter_mut() {
            let possible_command = event.execute();
            if possible_command.is_some() {
                commands.push(possible_command.unwrap());
            }

            if event.get_status() == &EventStatus::Failed {
                Logger::log(LogLevel::High, "EventManager", "An event failed to process...");
            }
        }

        self.events.retain(|e| e.get_status() != &EventStatus::Done);
        return commands;
    }

    pub fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }
}