use std::any::Any;
use crate::engine::utils::structs::transform::Transform;


pub trait Entity {
    fn get_id(&self) -> &usize;
    fn get_transform(&self) -> &Transform;
    fn modify_transform(&mut self, new_transform: Transform);
}

pub enum EntityCreateInfo {
    Entity(),
    PlayerEntity(Transform),
}