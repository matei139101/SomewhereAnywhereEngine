use std::sync::{Arc, Mutex};

use glam::vec3;

use crate::engine::{components::gamestage::{entities::{entity_manager::EntityManager, subcomponents::player_entity::PlayerEntityCreateInfo}, events::{event_manager::EventManager, subcomponents::render_object::RenderObject}}, vulkan::{structs::vertex::Vertex, vulkan_container::{self, VulkanContainer}}};
use crate::engine::utils::structs::transform::Transform;

pub struct GameStage {
    pub entity_manager: EntityManager,
    pub event_manager: EventManager,
    pub active_player_id: usize,
}

impl GameStage {
    pub fn new(vulkan_container: Arc<Mutex<VulkanContainer>>) -> Self {
        let cube = vec![
            // Front face (+Z)
            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 0.0, 0.0]), // bottom-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), [0.0, 0.0, 1.0]), // top-right
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 1.0, 0.0]), // bottom-right

            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 0.0, 0.0]), // bottom-left
            Vertex::new(vec3(-0.5,  0.5,  0.5), [1.0, 1.0, 0.0]), // top-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), [0.0, 0.0, 1.0]), // top-right

            // Back face (-Z)
            Vertex::new(vec3( 0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),

            // Left face (-X)
            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5,  0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3(-0.5, -0.5,  0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5,  0.5), [0.0, 0.0, 1.0]),

            // Right face (+X)
            Vertex::new(vec3(0.5, -0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3(0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(0.5, -0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5,  0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),

            // Top face (+Y)
            Vertex::new(vec3(-0.5, 0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5, 0.5, -0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3( 0.5, 0.5,  0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, 0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5, 0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5, 0.5, -0.5), [0.0, 0.0, 1.0]),

            // Bottom face (-Y)
            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3( 0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 0.0, 1.0]),
        ];

        let mut event_manager = EventManager::new(vulkan_container.clone());
        event_manager.add_event(Box::new(RenderObject::new(cube, vulkan_container)));

        let mut entity_manager = EntityManager::new();
        let player_transform = Transform::new(
            vec3(-1.0, -1.0, -5.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        );
        
        entity_manager.create_entity(Box::new(PlayerEntityCreateInfo::new(player_transform)));
        
        return GameStage { entity_manager, event_manager, active_player_id: 0 };
    }

    pub fn update(&mut self) {
        self.event_manager.process_frame();
        //self.entity_manager.process_frame();
    }
}