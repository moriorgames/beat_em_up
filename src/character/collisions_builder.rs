use super::box_collision::BoxCollision;

pub fn create_barbarian_body_collision(size_width: f32) -> BoxCollision {
    BoxCollision {
        x: 10.0,
        y: -125.0,
        w: size_width - 100.0,
        h: 125.0,
    }
}

pub fn create_barbarian_weapon_collision() -> BoxCollision {
    BoxCollision {
        x: 90.0,
        y: -85.0,
        w: 70.0,
        h: 60.0,
    }
}

pub fn create_orc_body_collision(size_width: f32) -> BoxCollision {
    BoxCollision {
        x: 10.0,
        y: -125.0,
        w: size_width - 75.0,
        h: 145.0,
    }
}

pub fn create_orc_weapon_collision_template() -> BoxCollision {
    BoxCollision {
        x: 90.0,
        y: -25.0,
        w: 80.0,
        h: 75.0,
    }
}

pub fn create_generic_foot_collision() -> BoxCollision {
    BoxCollision {
        x: 0.0,
        y: 75.0,
        w: 150.0,
        h: 50.0,
    }
}
