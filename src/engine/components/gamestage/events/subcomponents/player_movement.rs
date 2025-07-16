use glam::Vec3;
use crate::engine::components::gamestage::{entities::subcomponents::player_entity::PlayerEntity, events::subcomponents::event::{Event, EventStatus}};

pub struct PlayerMovementEvent {
    status: EventStatus,
    delta: Vec3,
}

impl Event for PlayerMovementEvent {
    fn execute(&mut self) {
        self.process();
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status
    }
}

impl PlayerMovementEvent {
    pub fn new(delta: Vec3, player_entity: &mut PlayerEntity) -> Self {
        return PlayerMovementEvent{
            status: EventStatus::Pending,
            delta,
        };
    }

    fn process(&self) {
        todo!()
    }
}