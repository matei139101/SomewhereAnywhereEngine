use std::collections::HashMap;

use crate::engine::components::entity_component::entities::entity::Entity;

pub struct EntityRepository {
    entities: HashMap<usize, Box<dyn Entity>>,
    unused_id: usize,
}

impl EntityRepository {
    pub fn new() -> Self {
        EntityRepository {
            entities: HashMap::new(),
            unused_id: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> &usize {
        //[TO-DO]: Add some error checking or logging
        self.entities.insert(self.unused_id, entity);
        self.unused_id += 1;

        &self.unused_id
    }

    pub fn remove_entity(&mut self, entity_id: &usize) {
        //[TO-DO]: Add some error checking or logging
        self.entities.remove(entity_id);
    }

    pub fn get_entity(&mut self, entity_id: &usize) -> &mut Box<dyn Entity> {
        //[TO-DO]: Add some error checking or logging
        self.entities.get_mut(entity_id).unwrap()
    }
}
