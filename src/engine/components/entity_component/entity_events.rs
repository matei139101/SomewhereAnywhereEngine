use crate::engine::{
    components::entity_component::entities::entity_enum::EntityType,
    utils::structs::transform::Transform,
};

pub struct CreateEntityEvent {
    pub entity_type: EntityType,
    pub transform: Transform,
}

pub struct DeleteEntityEvent {
    pub entity_id: usize,
}
