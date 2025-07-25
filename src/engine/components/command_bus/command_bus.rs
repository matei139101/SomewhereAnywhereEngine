use glam::Vec3;
use winit::keyboard::PhysicalKey;

use crate::engine::{components::{entities::{entity::EntityType, entity_manager::EntityManager}, events::{event_manager::EventManager, subcomponents::player_movement::PlayerMovementEvent}, gamestage::gamestage::GameStage, input_manager::input_manager::InputManager, vulkan_manager::vulkan_manager::VulkanManager}, utils::structs::transform::Transform, vulkan::structs::{vertex::Vertex, viewport::ViewportInfo}};

pub struct CommandBus {
    vulkan_manager: VulkanManager,
    event_manager: EventManager,
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
    pub fn new(vulkan_manager: VulkanManager, event_manager: EventManager, entity_manager: EntityManager, input_manager: InputManager, gamestage: GameStage) -> Self {
        return CommandBus {
            vulkan_manager,
            event_manager,
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
                self.event_manager.add_event(Box::new(PlayerMovementEvent::new(movement, camera, 0.03, 0.001, self.entity_manager.get_player_entity(player_id).clone())))
            },

            //Entity manager commands.
            CommandType::CreateEntity(create_info) => {self.entity_manager.create_entity(create_info);},
            CommandType::CreateEntityForPlayer() => {
                let mut front_of_player_transform = self.entity_manager.get_player_entity(0).lock().unwrap().get_transform().clone();
                front_of_player_transform.position = -front_of_player_transform.position + front_of_player_transform.forward() * 2.0;

                let new_cube_info: EntityType = EntityType::CubeEntity(front_of_player_transform, "src/engine/vulkan/base_resources/default_texture.png".to_string()); 
                self.entity_manager.create_entity(new_cube_info);
            },
            CommandType::DeleteLastEntity() => {
                let entities = self.entity_manager.get_entities();
                if let Some(last_entity_arc) = entities.last() {
                    let last_entity = last_entity_arc.lock().unwrap();
                    let last_entity_id = last_entity.get_id();
                    print!("Deleting entity with ID: {}", last_entity_id);
                    self.entity_manager.delete_entity(*last_entity_id);
                }
            },

            //Vulkan manager commands.
            CommandType::CreateVulkanObject(object_id, vertices, object_transform, texture_path) => {self.vulkan_manager.create_vulkan_object(object_id, vertices, object_transform, texture_path.as_str());},
            CommandType::DeleteVulkanObject(object_id) => {self.vulkan_manager.delete_vulkan_object(object_id);},
            CommandType::VulkanViewportResize(viewport_info) => {self.vulkan_manager.resize_viewport(viewport_info);},
        }
    }

    pub fn update_managers(&mut self) {
        for command in self.event_manager.process() {
            self.send_command(command);
        }

        for command in self.entity_manager.process() {
            self.send_command(command);
        }

        for command in self.input_manager.process() {
            self.send_command(command);
        }

        //[TO-DO]: Another rust moment forcing me to make a whole block for what could be a oneliner. Need to look into if this is "fixable"
        let binding = self.entity_manager.get_player_entity(0);
        let binding = binding.lock().unwrap();
        let viewport_transform = binding.get_transform();
        self.vulkan_manager.request_draw(viewport_transform.get_position(), viewport_transform.get_rotation());
    }
}