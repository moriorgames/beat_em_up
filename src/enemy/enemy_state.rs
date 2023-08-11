use crate::geometry::position::Position;
use crate::geometry::size::Size;

pub struct EnemyState {
    pub position: Position,
    pub size: Size,
    pub current_health: f32,
    pub max_health: f32,
}

impl EnemyState {
    pub const WIDTH: f32 = 40.0;
    pub const HEIGHT: f32 = 30.0;

    pub fn new() -> Self {
        EnemyState {
            position: Position::new(600.0, 250.0),
            size: Size::new(Self::WIDTH, Self::HEIGHT),
            current_health: 800.0,
            max_health: 1000.0,
        }
    }

    pub fn update_position(&mut self, position: Position) {
        if self.current_health > 1.0 {
            self.current_health -= 1.0;
        }
        self.position = position;
    }
}
