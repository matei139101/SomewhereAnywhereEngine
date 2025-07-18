use glam::Vec3;

pub struct GameStage {
    pub active_player_id: usize,
    pub starting_point: Vec3,
}

impl GameStage {
    pub fn new(active_player_id: usize, starting_point: Vec3) -> Self {
        return GameStage { active_player_id, starting_point };
    }

    pub fn update(&mut self) {
    }
}