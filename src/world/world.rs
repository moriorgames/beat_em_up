use crate::character::collision::Box;

pub struct World {
    pub bounds: Box,
}

impl World {
    pub fn new(bounds: Box) -> Self {
        World { bounds }
    }
}
