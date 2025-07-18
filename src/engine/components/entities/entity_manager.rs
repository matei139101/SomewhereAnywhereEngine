use std::sync::{Arc, Mutex};

use crate::engine::{components::entities::{entity::{Entity, EntityCreateInfo}, subcomponents::player_entity::PlayerEntity}, utils::structs::model::Model};

pub struct EntityManager {
    player_entities: Vec<Arc<Mutex<PlayerEntity>>>,
    entities: Vec<Arc<Mutex<Box<dyn Entity>>>>,
}

impl EntityManager {
    pub fn new() -> Self {
        return EntityManager{
            player_entities: vec![],
            entities: vec![],
        }
    }

    pub fn create_entity(&mut self, create_info: EntityCreateInfo) {
        match create_info {
            /*
            EntityCreateInfo::Entity() => {
                todo!();
            },
            */
            EntityCreateInfo::PlayerEntity(transform) => {
                let unreserved_id: usize = if self.player_entities.len() <= 0 {0} else {*self.entities.last().unwrap().lock().unwrap().get_id()};
                self.player_entities.push(Arc::new(Mutex::new(PlayerEntity::new(unreserved_id, transform))));
            }
        }
    }

    pub fn get_player_entity(&mut self, id: usize) -> Arc<Mutex<PlayerEntity>> {
        return self.player_entities[id].clone();
    }

    pub fn get_player_entity_ref(&self, id: usize) -> Arc<Mutex<PlayerEntity>> {
        return self.player_entities[id].clone();
    }

    pub fn get_entities(&self) -> Vec<Arc<Mutex<Box<dyn Entity>>>> {
        return self.entities.clone();
    }

    pub fn get_entity_models(&self) -> Vec<&Model> {
        let mut models: Vec<&Model> = vec![];

        for entity in self.entities.clone() {
            models.push(entity.lock().unwrap().get_model());
        }

        return models;
    }

    /*
    pub fn modify_entity_transform(&mut self, entity_id: usize, new_transform: Transform) {
        self.entities.iter_mut().find(|entity| entity.lock().unwrap().get_id() == &entity_id).unwrap().lock().unwrap().modify_transform(new_transform);
    }
    */
}