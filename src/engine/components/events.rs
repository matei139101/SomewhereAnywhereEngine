use crate::engine::{utils::logger::{LogLevel, Logger}, vulkan::structs::vertex::Vertex};

pub struct EventManager {
    events: Vec<Box<dyn Event>>
}

impl EventManager {
    pub fn new() -> Self {
        return EventManager { 
            events: vec![]
        }
    }

    pub fn actualize(&mut self) {
        for (index, event) in self.events.iter_mut().enumerate() {
            event.execute();
        }

        self.events.retain(|e| e.get_status() != &EventStatus::Done);
    }

    pub fn add_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push(Box::new(event));
    }
}

pub trait Event {
    fn execute(&mut self);
    fn get_status(&self) -> &EventStatus;
}

#[derive(PartialEq)]
pub enum EventStatus {
    Pending,
    Processing,
    Done,
    Failed,
}

pub struct RenderObject {
    status: EventStatus,
    object: Vec<Vertex>,
}

impl RenderObject {
    pub fn new(object: Vec<Vertex>) -> Self {
        return RenderObject {
            status: EventStatus::Pending,
            object: object 
        }
    }

    fn process(&mut self) {
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event starting process...");
        self.status = EventStatus::Processing;
        // Do something
        self.status = EventStatus::Done;
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event finished process.");
    }
}

impl Event for RenderObject {
    fn execute(&mut self) {
        self.process();
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status;
    }
}