use crate::engine::components::{gamestage::{entities::entity_manager::EntityManager, events::{event_manager::EventManager, subcomponents::event::Event}}};

pub struct CommandBus {
    entity_manager: EntityManager,
    event_manager: EventManager,
}

pub enum CommandType {
    Event(Box<dyn Event>),
}

impl CommandBus {
    pub fn new(entity_manager: EntityManager, event_manager: EventManager) -> Self {
        return CommandBus {
            entity_manager,
            event_manager,
        };
    }

    pub fn send_command(&mut self, command: CommandType) {
        match command {
            CommandType::Event(command) => {self.event_manager.add_event(command)},
            _ => {}
        }
    }
}