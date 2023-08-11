use crate::geometry::position::Position;

pub struct PlayerState {
    pub position: Position,
}

impl PlayerState {
    const SPEED: f32 = 3.7;

    pub fn new() -> Self {
        PlayerState {
            position: Position::new(400.0, 300.0),
        }
    }

    pub fn move_left(&mut self) {
        self.position.x -= Self::SPEED;
    }
    pub fn move_right(&mut self) {
        self.position.x += Self::SPEED;
    }
    pub fn move_up(&mut self) {
        self.position.y -= Self::SPEED;
    }
    pub fn move_down(&mut self) {
        self.position.y += Self::SPEED;
    }
}
