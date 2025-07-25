use std::sync::{Arc, Mutex};
use crate::engine::{components::{command_bus::command_bus::CommandType, entities::{entity::{Entity, EntityType}, subcomponents::{cube_entity::CubeEntity, player_entity::PlayerEntity}}}};

pub struct EntityManager {
    player_entities: Vec<Arc<Mutex<PlayerEntity>>>,
    entities: Vec<Arc<Mutex<Box<dyn Entity>>>>,
    buffered_commands: Vec<CommandType>,
}

impl EntityManager {
    pub fn new() -> Self {
        return EntityManager{
            player_entities: vec![],
            entities: vec![],
            buffered_commands: vec![],
        }
    }

    pub fn create_entity(&mut self, create_info: EntityType) {
        match create_info {
            EntityType::PlayerEntity(transform) => {
                let unreserved_id: usize = if self.player_entities.len() <= 0 {0} else {*self.player_entities.last().unwrap().lock().unwrap().get_id()};
                self.player_entities.push(Arc::new(Mutex::new(PlayerEntity::new(unreserved_id, transform))));
            },
            EntityType::CubeEntity(transform, texture_path) => {
                let unreserved_id: usize = if self.entities.len() <= 0 {0} else {*self.entities.last().unwrap().lock().unwrap().get_id() + 1};
                let cube_entity = CubeEntity::new(unreserved_id, transform.clone());
                self.buffered_commands.push(CommandType::CreateVulkanObject(unreserved_id, cube_entity.get_model().get_model().clone(), transform.clone(), texture_path));
                self.entities.push(Arc::new(Mutex::new(Box::new(cube_entity))));
            },
        }
    }

    pub fn delete_entity(&mut self, entity_id: usize) {
        if self.entities.get(entity_id).is_some() {
            self.entities.remove(entity_id);
            self.buffered_commands.push(CommandType::DeleteVulkanObject(entity_id));
        }
    }

    pub fn get_entities(&self) -> Vec<Arc<Mutex<Box<dyn Entity>>>> {
        return self.entities.clone();
    }

    pub fn get_player_entity(&mut self, id: usize) -> Arc<Mutex<PlayerEntity>> {
        return self.player_entities[id].clone();
    }

    pub fn process(&mut self) -> Vec<CommandType> {
        return std::mem::take(&mut self.buffered_commands);
    }
}