use crate::engine::{components::entities::entity::Entity, utils::structs::{model::Model, transform::Transform}};

pub struct CubeEntity {
    id: usize,
    transform: Transform,
    model: Model,
}

impl Entity for CubeEntity {
    fn get_id(&self) -> &usize {
        return &self.id;
    }

    fn get_model(&self) -> &Model {
        return &self.model;
    }
}

impl CubeEntity {
    pub fn new(id: usize, transform: Transform, model: Model) -> Self {
        return CubeEntity { 
            id,
            transform,
            model
        }
    }
}