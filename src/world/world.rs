use crate::geometry::{position::Position, size::Size};

pub struct World {
    pub position: Position,
    pub size: Size,
}

impl World {
    pub fn new(position: Position, size: Size) -> Self {
        World { position, size }
    }
}
