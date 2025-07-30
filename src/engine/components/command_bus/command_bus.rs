use glam::Vec3;
use winit::keyboard::PhysicalKey;

use crate::engine::{components::{entities::{entity::EntityType, entity_manager::EntityManager}, gamestage::gamestage::GameStage, input_manager::input_manager::InputManager, vulkan_manager::vulkan_manager::VulkanManager}, utils::structs::transform::Transform, vulkan::structs::{vertex::Vertex, viewport::ViewportInfo}};

pub struct CommandBus {
    vulkan_manager: VulkanManager,
    entity_manager: EntityManager,
    input_manager: InputManager,
    gamestage: GameStage,
}

#[derive(Debug)]
pub enum CommandType {
    KeyStateChange(PhysicalKey, bool),
    AxisStateChange(String, (f64, f64)),
    PlayerController(Vec3, (f64, f64), usize),
    CreateVulkanObject(usize, Vec<Vertex>, Transform, String),
    DeleteVulkanObject(usize),
    VulkanViewportResize(ViewportInfo),
    CreateEntity(EntityType),
    
    //[TO-DO]: For testing and cool purposes.
    CreateEntityForPlayer(),
    DeleteLastEntity(),
}

impl CommandBus {
    pub fn new(vulkan_manager: VulkanManager, entity_manager: EntityManager, input_manager: InputManager, gamestage: GameStage) -> Self {
        return CommandBus {
            vulkan_manager,
            entity_manager,
            input_manager,
            gamestage,
        };
    }

    pub fn send_command(&mut self, command: CommandType) {
        match command {
            //Input manager commands.
            CommandType::KeyStateChange(key, state) => {self.input_manager.key_event(key, state);},
            CommandType::AxisStateChange(axis, value) => {self.input_manager.axis_event(axis, value.0, value.1);},

            //Event manager commands.
            CommandType::PlayerController(movement, camera, player_id) => {
                let player_entity = self.entity_manager.get_player_entity(player_id);

                let mut new_transform: Transform = player_entity.get_transform().clone();

                //Movement
                let movement_delta = 
                    new_transform.forward() * movement.z * 0.03 +  // Forward/backward
                    new_transform.right() * movement.x * 0.03 +    // Left/right
                    new_transform.up() * movement.y * 0.03;        // Up/down
    
                new_transform.position = new_transform.get_position() + movement_delta;

                //Camera
                new_transform.rotation.y += camera.0 as f32 * 0.001;
                new_transform.rotation.x += camera.1 as f32 * -0.001;
                new_transform.rotation.x = new_transform.get_rotation().x.clamp(-1.5, 1.5);

                player_entity.modify_transform(new_transform);
            },

            //Entity manager commands.
            CommandType::CreateEntity(create_info) => {self.entity_manager.create_entity(create_info);},
            CommandType::CreateEntityForPlayer() => {
                let mut front_of_player_transform = self.entity_manager.get_player_entity(0).get_transform().clone();
                front_of_player_transform.position = -front_of_player_transform.position + front_of_player_transform.forward() * 2.0;

                let new_cube_info: EntityType = EntityType::CubeEntity(front_of_player_transform, "src/engine/vulkan/base_resources/default_texture.png".to_string()); 
                self.entity_manager.create_entity(new_cube_info);
            },
            CommandType::DeleteLastEntity() => {
                let entities = self.entity_manager.get_entities();
                if let Some(id) = entities.keys().max() {
                    let id = *id;
                    print!("Deleting entity with ID: {}", id);
                    self.entity_manager.delete_entity(&id);
                    self.vulkan_manager.delete_vulkan_object(id);
                }
            },

            //Vulkan manager commands.
            CommandType::CreateVulkanObject(object_id, vertices, object_transform, texture_path) => {self.vulkan_manager.create_vulkan_object(object_id, vertices, object_transform, texture_path.as_str());},
            CommandType::DeleteVulkanObject(object_id) => {self.vulkan_manager.delete_vulkan_object(object_id);},
            CommandType::VulkanViewportResize(viewport_info) => {self.vulkan_manager.resize_viewport(viewport_info);},
        }
    }

    pub fn update_managers(&mut self) {
        for command in self.entity_manager.process() {
            self.send_command(command);
        }

        for command in self.input_manager.process() {
            self.send_command(command);
        }

        //[TO-DO]: This feels like spaghetti code...
        let player_entity = self.entity_manager.get_player_entity(0);
        let viewport_transform = player_entity.get_transform();
        self.vulkan_manager.request_draw(viewport_transform.get_position(), viewport_transform.get_rotation());
    }
}