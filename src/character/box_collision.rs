use crate::geometry::{position::Position, size::Size};

#[derive(Clone)]
pub struct CollisionBox {
    pub position: Position,
    pub size: Size,
}
