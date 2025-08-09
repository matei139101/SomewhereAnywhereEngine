use std::any::Any;

use glam::{vec2, vec3};

use crate::engine::{components::entities::entity::{Entity, EntityCommand}, utils::structs::{model::Model, transform::Transform}, vulkan::structs::vertex::Vertex};

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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn recieve_command(&mut self, entity_command: EntityCommand) {
        // Nothing to do here.
    }
}

impl CubeEntity {
    pub fn new(id: usize, transform: Transform) -> Self {
        let model = Model::new(vec![
            // Front face (+Z)
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)), // bottom-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)), // top-right
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)), // bottom-right

            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)), // bottom-left
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)), // top-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)), // top-right

            // Back face (-Z)
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),

            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),
            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),

            // Left face (-X)
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),

            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),

            // Right face (+X)
            Vertex::new(vec3(0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),
            Vertex::new(vec3(0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),

            Vertex::new(vec3(0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(0.5,  0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),
            Vertex::new(vec3(0.5,  0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),

            // Top face (+Y)
            Vertex::new(vec3(-0.5, 0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3( 0.5, 0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),
            Vertex::new(vec3( 0.5, 0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),

            Vertex::new(vec3(-0.5, 0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(-0.5, 0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),
            Vertex::new(vec3( 0.5, 0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),

            // Bottom face (-Y)
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),

            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(255.0, 255.0, 255.0), vec2(0.0, 0.0)),
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 0.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(255.0, 255.0, 255.0), vec2(1.0, 1.0)),
        ]);
        
        return CubeEntity { 
            id,
            transform,
            model
        }
    }
}