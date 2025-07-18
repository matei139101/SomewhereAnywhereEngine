use std::sync::{Arc, Mutex};

use glam::Vec3;
use winit::keyboard::PhysicalKey;

use crate::engine::components::{entities::{entity_manager::{EntityManager}}, events::{event_manager::EventManager, subcomponents::{event::Event, player_movement::PlayerMovementEvent}}, input_manager::{input_manager::InputManager}};

pub struct CommandBus {
    event_manager: EventManager,
    entity_manager: EntityManager,
    input_manager: InputManager,
}

pub enum CommandType {
    Event(Box<dyn Event>),
    KeyStateChange(PhysicalKey, bool),
    AxisStateChange(String, (f64, f64)),
    PlayerController(Vec3, (f64, f64), usize),
}

impl CommandBus {
    pub fn new(event_manager: EventManager, entity_manager: EntityManager, input_manager: InputManager) -> Self {
        return CommandBus {
            event_manager,
            entity_manager,
            input_manager,
        };
    }

    pub fn get_entity_manager(&self) -> &EntityManager {
        return &self.entity_manager;
    }

    pub fn send_command(&mut self, command: CommandType) {
        match command {
            CommandType::Event(command) => {self.event_manager.add_event(command)},
            CommandType::KeyStateChange(key, state) => {self.input_manager.key_event(key, state);},
            CommandType::AxisStateChange(axis, value) => {self.input_manager.axis_event(value.0, value.1);},
            CommandType::PlayerController(movement, camera, player_id) => {
                self.event_manager.add_event(Box::new(PlayerMovementEvent::new(movement, camera, 0.03, 0.01, self.entity_manager.get_player_entity(player_id).clone())))
            },
        }
    }

    pub fn update_managers(&mut self) {
        self.event_manager.process();

        for command in self.input_manager.process() {
            self.send_command(command);
        }
    }
}