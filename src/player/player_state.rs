use nalgebra::Point2;

pub struct PlayerState {
    pub position: Point2<f32>,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            position: Point2::new(400.0, 300.0),
        }
    }

    pub fn move_right(&mut self) {
        self.position.x += 2.0;
    }
}
