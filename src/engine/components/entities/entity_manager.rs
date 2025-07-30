use std::collections::HashMap;
use crate::engine::components::{command_bus::command_bus::CommandType, entities::{entity::{Entity, EntityType}, subcomponents::{cube_entity::CubeEntity, player_entity::PlayerEntity}}};

pub struct EntityManager {
    entities: HashMap<usize, Box<dyn Entity>>,
    buffered_commands: Vec<CommandType>,
    next_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        return EntityManager{
            entities: HashMap::new(),
            buffered_commands: vec![],
            next_id: 0,
        }
    }

    pub fn create_entity(&mut self, create_info: EntityType) {
        match create_info {
            EntityType::PlayerEntity(transform) => {
                self.entities.insert(self.next_id, Box::new(PlayerEntity::new(self.next_id, transform)));
            },
            EntityType::CubeEntity(transform, texture_path) => {
                let cube_entity = CubeEntity::new(self.next_id, transform.clone());
                self.buffered_commands.push(CommandType::CreateVulkanObject(self.next_id, cube_entity.get_model().get_model().clone(), transform.clone(), texture_path));
                self.entities.insert(self.next_id, Box::new(cube_entity));
            },
        }

        self.next_id += 1;
    }

    pub fn delete_entity(&mut self, entity_id: &usize) {
        self.entities.remove(entity_id);
    }

    //[TO-DO]: This shouldnt exist.
    pub fn get_entities(&self) -> &HashMap<usize, Box<dyn Entity>> {
        return &self.entities;
    }

    //[TO-DO]: Ideally this needs to be reworked again in the future to make it so the entity manager doesn't need to figure out the players entity id itself.
    pub fn get_player_entity(&mut self, id: usize) -> &mut PlayerEntity {
        let mut player_entity_ids: Vec<usize> = vec![];
        for (index, entity) in self.entities.iter_mut() {
            let casted_entity = entity.as_any_mut();
            if casted_entity.is::<PlayerEntity>() {
                player_entity_ids.push(*index);
            }
        }

        return self.entities.get_mut(&player_entity_ids[id]).unwrap().as_any_mut().downcast_mut::<PlayerEntity>().unwrap();
    }

    pub fn process(&mut self) -> Vec<CommandType> {
        return std::mem::take(&mut self.buffered_commands);
    }
}