use nalgebra::Point2;

pub struct EnemyState {
    pub position: Point2<f32>,
    pub current_health: f32,
    pub max_health: f32,
}

impl EnemyState {
    pub fn new() -> Self {
        EnemyState {
            position: Point2::new(600.0, 250.0),
            current_health: 500.0,
            max_health: 1000.0,
        }
    }

    pub fn update_position(&mut self, position: Point2<f32>) {
        if self.current_health > 1.0 {
            self.current_health -= 1.0;
        }
        self.position = position;
    }
}
