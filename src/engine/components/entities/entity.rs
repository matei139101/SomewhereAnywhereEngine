use std::any::Any;
use crate::engine::utils::structs::transform::Transform;

pub trait Entity {
    fn get_id(&self) -> &usize;
    fn as_any(&mut self) -> &mut dyn Any;
    fn get_transform(&self) -> &Transform;
    fn modify_transform(&mut self, new_transform: Transform);
}

pub trait EntityCreateInfo {
    fn create_from_info(&self, id: usize) -> Box<dyn Entity>;
}