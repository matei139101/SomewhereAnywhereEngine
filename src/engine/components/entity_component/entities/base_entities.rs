use glam::{vec2, vec3};

use crate::engine::{
    components::entity_component::entities::entity::{Entity, HasModel},
    utils::structs::{model::Model, transform::Transform},
    vulkan::structs::vertex::Vertex,
};

pub struct PlayerEntity {
    transform: Transform,
}

impl PlayerEntity {
    pub fn new(transform: Transform) -> Self {
        PlayerEntity { transform }
    }

    pub fn get_camera_transform(&mut self) -> Transform {
        //[TO-DO]: Change this to work with a camera trait or component or whatever in the future.
        let mut camera_transform = self.transform.clone();
        camera_transform.position = vec3(
            camera_transform.position.x,
            camera_transform.position.y + 1.0,
            camera_transform.position.z,
        );

        camera_transform
    }
}

impl Entity for PlayerEntity {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

pub struct CubeEntity {
    transform: Transform,
    model: Model,
}

impl CubeEntity {
    pub fn new(transform: Transform) -> Self {
        let model = Model::new(vec![
            // Front face (+Z)
            Vertex::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ), // bottom-left
            Vertex::new(
                vec3(0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ), // top-right
            Vertex::new(
                vec3(0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ), // bottom-right
            Vertex::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ), // bottom-left
            Vertex::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ), // top-left
            Vertex::new(
                vec3(0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ), // top-right
            // Back face (-Z)
            Vertex::new(
                vec3(0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 1.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 1.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            // Left face (-X)
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            // Right face (+X)
            Vertex::new(
                vec3(0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            Vertex::new(
                vec3(0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            // Top face (+Y)
            Vertex::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, 0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            // Bottom face (-Y)
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
            Vertex::new(
                vec3(0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(0.0, 0.0),
            ),
            Vertex::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 0.0),
            ),
            Vertex::new(
                vec3(0.5, -0.5, 0.5),
                vec3(255.0, 255.0, 255.0),
                vec2(1.0, 1.0),
            ),
        ]);

        CubeEntity { transform, model }
    }
}

impl Entity for CubeEntity {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

impl HasModel for CubeEntity {
    fn get_model(&self) -> &Model {
        &self.model
    }
}
