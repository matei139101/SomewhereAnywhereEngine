use std::any::Any;

use crate::engine::utils::structs::{model::Model, transform::Transform};

pub trait Entity: Any + Send + Sync {
    fn get_transform(&self) -> &Transform;
    fn set_transform(&mut self, new_transform: Transform);
}

pub trait HasModel {
    fn get_model(&self) -> &Model;
}
