use crate::engine::{components::entities::entity::{Entity}, utils::structs::transform::Transform};

#[derive(Debug)]
pub struct PlayerEntity {
    id: usize,
    transform: Transform,
}

impl Entity for PlayerEntity {
    fn get_id(&self) -> &usize {
        return &self.id;
    }

    fn get_transform(&self) -> &Transform {
        return &self.transform;
    }

    fn modify_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

impl PlayerEntity {
    pub fn new(id: usize, transform: Transform) -> Self {
        return PlayerEntity{
            id,
            transform,
        };
    }
}