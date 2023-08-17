use crate::character::box_collision::BoxCollision;

pub struct World {
    pub bounds: BoxCollision,
}

impl World {
    pub fn new(bounds: BoxCollision) -> Self {
        World { bounds }
    }
}
