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

    pub fn process(&mut self) {
        for event in self.events.iter_mut() {
            event.execute();

            if event.get_status() == &EventStatus::Failed {
                Logger::log(LogLevel::High, "EventManager", "An event failed to process...");
            }
        }

        self.events.retain(|e| e.get_status() != &EventStatus::Done);
    }

    pub fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }
}