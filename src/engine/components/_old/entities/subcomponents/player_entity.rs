use std::{any::Any, default};

use crate::engine::{components::entities::entity::{Entity, EntityCommand}, utils::structs::{model::Model, transform::Transform}};

#[derive(Debug)]
pub struct PlayerEntity {
    id: usize,
    transform: Transform,
    model: Model,
    speed: f32,
    sensitivity: f32,
}

impl PlayerEntity {
    pub fn new(id: usize, transform: Transform) -> Self {
        return PlayerEntity{
            id,
            transform,
            model: Model::new(vec![]),
            speed: 0.03f32,
            sensitivity: 0.001f32,
        };
    }

    pub fn get_id(&self) -> &usize {
        return &self.id;
    }

    pub fn get_transform(&self) -> &Transform {
        return &self.transform;
    }

    pub fn modify_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

impl Entity for PlayerEntity {
    fn get_id(&self) -> &usize {
        return &self.id;
    }

    fn get_model(&self) -> &Model {
        return &self.model;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn recieve_command(&mut self, entity_command: EntityCommand) {
        match entity_command {
            EntityCommand::MovePlayerEntity(velocity) => {
                let mut new_transform: Transform = self.transform.clone();

                //Movement
                let movement_delta = 
                    new_transform.forward() * velocity.z * self.speed +  // Forward/backward
                    new_transform.right() * velocity.x * self.speed +    // Left/right
                    new_transform.up() * velocity.y * self.speed;        // Up/down
    
                new_transform.position = new_transform.get_position() + movement_delta;

                self.transform = new_transform;
            },
            EntityCommand::TurnPlayerEntity(x, y) => {
                let mut new_transform: Transform = self.transform.clone();

                //Camera
                new_transform.rotation.y += y as f32 * self.sensitivity;
                new_transform.rotation.x += x as f32 * -self.sensitivity;
                new_transform.rotation.x = new_transform.get_rotation().x.clamp(-1.5, 1.5);

                self.transform = new_transform;
            },
            default => {
                //[TO-DO]: Add some handling to give a heads up the command reached a reciever which doesn't handle this command
            }
        }
    }
}