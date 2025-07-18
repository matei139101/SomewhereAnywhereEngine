use crate::engine::{components::entities::entity::Entity, utils::structs::transform::Transform, vulkan::structs::vertex::Vertex};

pub struct CubeEntity {
    id: usize,
    transform: Transform,
    model: Vec<Vertex>,
}

impl Entity for CubeEntity {
    fn get_id(&self) -> &usize {
        return &self.id;
    }

    fn get_transform(&self) -> &crate::engine::utils::structs::transform::Transform {
        return &self.transform;
    }

    fn get_model(&self) -> &crate::engine::utils::structs::model::Model {
        return self.get_model();
    }

    fn modify_transform(&mut self, new_transform: crate::engine::utils::structs::transform::Transform) {
        self.transform = new_transform
    }
}