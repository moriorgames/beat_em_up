pub struct PlayerState {
    health: i32,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            health: 100,
        }
    }

    pub fn update(&mut self) {
        self.health += 1;
    }
}
