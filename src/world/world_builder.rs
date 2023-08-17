pub mod world_builder {
    use crate::{character::box_collision::BoxCollision, world::world::World};

    pub fn build() -> World {
        let bounds: BoxCollision = BoxCollision {
            x: 5.0,
            y: 265.0,
            w: 1900.0,
            h: 775.0,
        };

        World::new(bounds)
    }
}
