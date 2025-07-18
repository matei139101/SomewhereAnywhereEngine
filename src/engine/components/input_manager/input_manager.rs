use std::{collections::HashMap};
use glam::vec3;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::engine::components::{command_bus::command_bus::CommandType};

pub struct InputManager {
    mapped_keys: HashMap<PhysicalKey, bool>,
    mapped_axes: HashMap<String, (f64, f64)>,
    player_id: usize,
}

impl InputManager {
    pub fn new(keys: Vec<PhysicalKey>, axes: Vec<String>, player_id: usize) -> Self {
        let mut mapped_keys: HashMap<PhysicalKey, bool> = HashMap::new();
        for key in keys {
            mapped_keys.insert(key, false);
        }

        let mut mapped_axes: HashMap<String, (f64, f64)> = HashMap::new();
        for axis in axes {
            mapped_axes.insert(axis, (0.0, 0.0));
        }

        return InputManager {
            mapped_keys,
            mapped_axes,
            player_id,
        };
    }

    pub fn key_event(&mut self, key: PhysicalKey, action: bool) {
        if self.mapped_keys.contains_key(&key) {
            self.mapped_keys.insert(key, action);
        };
    }

    //[TO-DO]: Will need to be expanded to support different axes in the future. e.g. controller joysticks...
    pub fn axis_event(&mut self, axis: String, x: f64, y: f64) {
        if self.mapped_axes.contains_key(&axis) {
            self.mapped_axes.insert("mouse".to_string(), (self.mapped_axes.get(&axis).unwrap().0 + x, self.mapped_axes.get(&axis).unwrap().1 + y));
        }
    }

    pub fn process(&mut self) -> Vec<CommandType> {
        let mut commands: Vec<CommandType> = vec![];
        for (key, &state) in &self.mapped_keys {
            match state {
                true => {
                    match key {
                        PhysicalKey::Code(KeyCode::KeyW) => {commands.push(CommandType::PlayerController(vec3(0.0, 0.0, -1.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::KeyA) => {commands.push(CommandType::PlayerController(vec3(1.0, 0.0, 0.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::KeyS) => {commands.push(CommandType::PlayerController(vec3(0.0, 0.0, 1.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::KeyD) => {commands.push(CommandType::PlayerController(vec3(-1.0, 0.0, 0.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::ControlLeft) => {commands.push(CommandType::PlayerController(vec3(0.0, -1.0, 0.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::Space) => {commands.push(CommandType::PlayerController(vec3(0.0, 1.0, 0.0), (0.0, 0.0), self.player_id))},
                        PhysicalKey::Code(KeyCode::KeyE) => {commands.push(CommandType::CreateEntityForPlayer())},
                        PhysicalKey::Code(KeyCode::KeyQ) => {commands.push(CommandType::DeleteLastEntity())},
                        _ => {},
                    }
                }
                false => continue,
            }
        }

        //[TO-DO]: VERY JITTERY. NEEDS TO BE GONE!!!
        let mut axes_to_reset = Vec::new();
        for (axis, &value) in &self.mapped_axes {
            if value.0 != 0.0 || value.1 != 0.0 {
                match axis.as_str() {
                    "mouse" => {commands.push(CommandType::PlayerController(vec3(0.0, 0.0, 0.0), value, self.player_id))}
                    _ => {}
                }

                axes_to_reset.push(axis.clone());
            }
        }
        for axis in axes_to_reset {
            self.mapped_axes.insert(axis, (0.0, 0.0));
        }

        return commands;
    }
}