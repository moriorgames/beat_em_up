use crate::geometry::position::Position;
use crate::geometry::size::Size;

pub struct PlayerState {
    pub position: Position,
    pub size: Size,
}

impl PlayerState {
    pub const W: f32 = 55.0;
    pub const H: f32 = 45.0;
    const SPEED: f32 = 3.7;

    pub fn new() -> Self {
        PlayerState {
            position: Position::new(400.0, 300.0),
            size: Size::new(Self::W, Self::H),
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
