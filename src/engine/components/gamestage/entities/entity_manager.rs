use crate::engine::{components::gamestage::entities::{entity::{Entity, EntityCreateInfo}, subcomponents::player_entity::PlayerEntity}, utils::structs::transform::Transform};

pub struct EntityManager {
    entities: Vec<Box<dyn Entity>>
}

impl EntityManager {
    pub fn new() -> Self {
        return EntityManager{entities: vec![]};
    }

    pub fn create_entity(&mut self, create_info: Box<dyn EntityCreateInfo>) {
        let unreserved_id: usize = if self.entities.len() <= 0 {0} else {*self.entities.last().unwrap().get_id()};
        let entity: Box<dyn Entity> = create_info.create_from_info(unreserved_id);
        self.entities.push(entity);
    }

    pub fn get_player_entities(&mut self) -> Vec<&mut PlayerEntity> {
        return self.entities
            .iter_mut()
            .filter_map(|entity| entity.as_any().downcast_mut::<PlayerEntity>())
            .collect()
    }

    pub fn modify_entity_transform(&mut self, entity_id: usize, new_transform: Transform) {
        self.entities.iter_mut().find(|entity| entity.get_id() == &entity_id).unwrap().modify_transform(new_transform);
    }
}