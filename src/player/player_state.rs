use nalgebra::Point2;

pub struct PlayerState {
    pub position: Point2<f32>,
}

impl PlayerState {
    const SPEED: f32 = 2.0;

    pub fn new() -> Self {
        PlayerState {
            position: Point2::new(400.0, 300.0),
        }
    }

    pub fn move_left(&mut self) {
        self.position.x -= Self::SPEED;
    }
    pub fn move_right(&mut self) {
        self.position.x += Self::SPEED;
    }
    pub fn move_up(&mut self) {
        self.position.y -= Self::SPEED;
    }
    pub fn move_down(&mut self) {
        self.position.y += Self::SPEED;
    }
}
