use nalgebra::Point2;

pub struct EnemyState {
    pub position: Point2<f32>,
}

impl EnemyState {
    pub fn new() -> Self {
        EnemyState {
            position: Point2::new(600.0, 250.0),
        }
    }
}
