use std::any::Any;

use crate::engine::{components::entities::entity::Entity, utils::structs::{model::Model, transform::Transform}};

#[derive(Debug)]
pub struct PlayerEntity {
    id: usize,
    transform: Transform,
    model: Model,
}

impl PlayerEntity {
    pub fn new(id: usize, transform: Transform) -> Self {
        return PlayerEntity{
            id,
            transform,
            model: Model::new(vec![]),
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
}