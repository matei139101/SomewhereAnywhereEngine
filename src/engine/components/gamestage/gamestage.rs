pub struct GameStage {
    player_id: usize,
}

impl GameStage {
    pub fn new(player_id: usize) -> Self {
        return GameStage {
            player_id,
        };
    }

    pub fn update(&mut self) {
    }
}