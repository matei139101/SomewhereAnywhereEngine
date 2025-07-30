use std::any::Any;

use crate::engine::utils::structs::{model::Model, transform::Transform};


pub trait Entity: Any {
    fn get_id(&self) -> &usize;
    fn get_model(&self) -> &Model;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub enum EntityType {
    PlayerEntity(Transform),
    CubeEntity(Transform, String),
}