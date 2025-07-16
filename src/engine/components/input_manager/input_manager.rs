use std::collections::HashMap;
use glam::vec3;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::engine::components::gamestage::{entities::subcomponents::player_entity::PlayerEntity, events::{event_manager::{self, EventManager}, subcomponents::player_movement::PlayerMovementEvent}};

pub struct InputManager<'a> {
    mapped_keys: HashMap<PhysicalKey, bool>,
    player_entity: &'a mut PlayerEntity,
}

impl InputManager {
    pub fn new(keys: Vec<PhysicalKey>, player_entity: &mut PlayerEntity) -> Self {
        let mut mapped_keys: HashMap<PhysicalKey, bool> = HashMap::new();
        for key in keys {
            mapped_keys.insert(key, false);
        }

        return InputManager {
            mapped_keys,
            player_entity,
        };
    }

    pub fn key_event(&mut self, key: PhysicalKey, action: bool) {
        if self.mapped_keys.contains_key(&key) {
            self.mapped_keys.insert(key, action);
        };
    }

    pub fn process(&self, event_manager: &mut EventManager) {
        for (key, &state) in &self.mapped_keys {
            match state {
                true => {
                    match key {
                        PhysicalKey::Code(KeyCode::KeyW) => {event_manager.add_event(*Box::new(PlayerMovementEvent::new(vec3(0.0, 0.0, 1.0))))},
                        PhysicalKey::Code(KeyCode::KeyA) => {event_manager.add_event(*Box::new(PlayerMovementEvent::new(vec3(1.0, 0.0, 0.0))))},
                        PhysicalKey::Code(KeyCode::KeyS) => {event_manager.add_event(*Box::new(PlayerMovementEvent::new(vec3(0.0, 0.0, -1.0))))},
                        PhysicalKey::Code(KeyCode::KeyD) => {event_manager.add_event(*Box::new(PlayerMovementEvent::new(vec3(-1.0, 0.0, 0.0))))},
                        _ => {},
                    }
                }
                false => continue, // Skip false values
            }
        }
    }
}