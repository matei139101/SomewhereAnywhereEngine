use std::sync::{Arc, Mutex};

use crate::engine::{components::entities::entity::{self, Entity}, vulkan::vulkan_container::{self, VulkanContainer}};

pub struct EntityManager {
    vulkan_container: Arc<Mutex<VulkanContainer>>,
    entities: Vec<Entity>
}

impl EntityManager {
    pub fn new(vulkan_container: Arc<Mutex<VulkanContainer>>) -> Self {
        return EntityManager{vulkan_container, entities: vec![]};
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, enitity_id: usize) {
        for (index, entity) in self.entities.iter().enumerate() {
            if entity.get_id() == &enitity_id {
                // todo
            }
        }
    }

    pub fn process(&self) {
        // Nothing yet, physics and stuff later probably...
    }
}