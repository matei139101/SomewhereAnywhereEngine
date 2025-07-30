use crate::engine::vulkan::structs::vertex::Vertex;

#[derive(Debug)]
pub struct Model {
    model: Vec<Vertex>,
}

impl Model {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        return Model { model: vertices };
    }

    pub fn get_model(&self) -> &Vec<Vertex> {
        return &self.model;
    }
}