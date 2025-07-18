use std::sync::{Arc, Mutex};

use glam::Vec3;
use crate::engine::{components::{entities::{entity::Entity, subcomponents::player_entity::PlayerEntity}, events::subcomponents::event::{Event, EventStatus}}, utils::structs::transform::Transform};

pub struct PlayerMovementEvent {
    status: EventStatus,
    movement: Vec3,
    camera: (f64, f64),
    speed: f32,
    sensitivity: f32,
    player_entity: Arc<Mutex<PlayerEntity>>,
}

impl Event for PlayerMovementEvent {
    fn execute(&mut self) {
        self.process();
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status
    }
}

impl PlayerMovementEvent {
    pub fn new(movement: Vec3, camera: (f64, f64), speed: f32, sensitivity: f32, player_entity: Arc<Mutex<PlayerEntity>>) -> Self {
        return PlayerMovementEvent{
            status: EventStatus::Pending,
            movement,
            camera,
            speed,
            sensitivity,
            player_entity,
        };
    }

    fn process(&mut self) {
        let mut new_transform: Transform = self.player_entity.lock().unwrap().get_transform().clone();

        //Movement
        let movement_delta = 
            new_transform.forward() * self.movement.z * self.speed +  // Forward/backward
            new_transform.right() * self.movement.x * self.speed +    // Left/right
            new_transform.up() * self.movement.y * self.speed;        // Up/down
    
        new_transform.position = new_transform.get_position() + movement_delta;

        //Camera
        new_transform.rotation.y += self.camera.0 as f32 * self.sensitivity;
        new_transform.rotation.x += self.camera.1 as f32 * -self.sensitivity;
        new_transform.rotation.x = new_transform.get_rotation().x.clamp(-1.5, 1.5);

        self.player_entity.lock().unwrap().modify_transform(new_transform);
        self.status = EventStatus::Done;
    }
}