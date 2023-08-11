use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

pub struct EnemyState {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub current_health: f32,
    pub max_health: f32,
}

impl EnemyState {
    pub const W: f32 = 40.0;
    pub const H: f32 = 30.0;
    const SPEED: f32 = 1.3;

    pub fn new() -> Self {
        EnemyState {
            id: Uuid::new_v4(),
            position: Position::new(600.0, 250.0),
            size: Size::new(Self::W, Self::H),
            current_health: 800.0,
            max_health: 1000.0,
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
