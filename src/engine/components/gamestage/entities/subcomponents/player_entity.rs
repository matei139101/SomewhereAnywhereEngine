use std::any::Any;

use glam::Vec3;

use crate::engine::{components::gamestage::entities::entity::{Entity, EntityCreateInfo}, utils::structs::transform::{self, Transform}};

pub struct PlayerEntity {
    id: usize,
    transform: Transform,
}

impl Entity for PlayerEntity {
    fn get_id(&self) -> &usize {
        return &self.id;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        return self;
    }

    fn get_transform(&self) -> &Transform {
        return &self.transform;
    }

    fn modify_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

impl PlayerEntity {
    pub fn move_forward(&mut self, delta: f32) {
        self.transform.position = self.transform.position + self.transform.forward() * -delta;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.transform.position = self.transform.position + self.transform.right() * -delta;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.transform.position = self.transform.position + self.transform.up() * delta;
    }
}

pub struct PlayerEntityCreateInfo {
    transform: Transform,
}

impl EntityCreateInfo for PlayerEntityCreateInfo {
    fn create_from_info(&self, id: usize) -> Box<dyn Entity> {
        return Box::new(PlayerEntity{id, transform: self.transform.clone()});
    }
}

impl PlayerEntityCreateInfo {
    pub fn new(transform: Transform) -> Self {
        return PlayerEntityCreateInfo { transform }
    }
}