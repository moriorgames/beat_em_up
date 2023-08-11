use crate::behaviours::movable::Movable;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

pub struct PlayerState {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub speed: f32,
    pub current_health: f32,
    pub max_health: f32,
}

impl PlayerState {
    pub const W: f32 = 55.0;
    pub const H: f32 = 45.0;

    pub fn new() -> Self {
        PlayerState {
            id: Uuid::new_v4(),
            position: Position::new(400.0, 300.0),
            size: Size::new(Self::W, Self::H),
            speed: 3.7,
            current_health: 800.0,
            max_health: 1000.0,
        }
    }
}

impl Movable for PlayerState {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn speed(&self) -> f32 {
        self.speed
    }
}
