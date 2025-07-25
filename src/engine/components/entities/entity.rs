use crate::engine::utils::structs::{model::Model, transform::Transform};


pub trait Entity {
    fn get_id(&self) -> &usize;
    fn get_model(&self) -> &Model;
}

#[derive(Debug)]
pub enum EntityType {
    PlayerEntity(Transform),
    CubeEntity(Transform, String),
}