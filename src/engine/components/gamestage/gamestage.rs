use crate::engine::components::{entities::entity_manager::{EntityManager}, events::event_manager::{EventManager}};

pub struct GameStage {
    pub entity_manager: EntityManager,
    pub event_manager: EventManager,
    pub active_player_id: usize,
}

impl GameStage {
    pub fn new(entity_manager: EntityManager, event_manager: EventManager) -> Self {
        return GameStage { entity_manager, event_manager, active_player_id: 0 };
    }

    pub fn update(&mut self) {
        self.event_manager.process_frame();
        //self.entity_manager.process_frame();
    }
}