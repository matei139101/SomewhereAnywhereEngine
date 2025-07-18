use crate::engine::{utils::structs::transform::Transform};

#[derive(Debug)]
pub struct PlayerEntity {
    id: usize,
    transform: Transform,
}

impl PlayerEntity {
    pub fn new(id: usize, transform: Transform) -> Self {
        return PlayerEntity{
            id,
            transform,
        };
    }

    pub fn get_transform(&self) -> &Transform {
        return &self.transform;
    }

    pub fn modify_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}