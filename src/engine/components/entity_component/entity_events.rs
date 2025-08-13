use crate::engine::utils::structs::transform::Transform;

pub struct EntityCreationInfo {
    pub transform: Transform,
}

pub struct EntityDeletionInfo {
    pub entity_id: usize,
}
