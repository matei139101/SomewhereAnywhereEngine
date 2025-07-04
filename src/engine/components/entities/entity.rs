use glam::Vec3;

use crate::engine::vulkan::structs::vertex::Vertex;

pub struct Entity {
    id: usize,
    position: Vec3,
    rotation: Vec3,
    size: Vec3,
    model: Vec<Vertex>
}

impl Entity {
    pub fn new(id: usize, position: Vec3, rotation: Vec3, size: Vec3, model: Vec<Vertex>) -> Self {
        return Entity { id, position, rotation, size, model }
    }

    pub fn get_id(&self) -> &usize {
        return &self.id;
    }

    pub fn get_position(&self) -> &Vec3 {
        return &self.position;
    } 

    pub fn get_rotation(&self) -> &Vec3 {
        return &self.rotation;
    }

    pub fn get_size(&self) -> &Vec3 {
        return &self.size;
    }

    pub fn get_model(&self) -> &Vec<Vertex> {
        return &self.model;
    }
}