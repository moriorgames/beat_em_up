#[derive(Clone)]
pub struct Size {
    pub height: f32,
    pub width: f32,
}

impl Size {
    pub fn new(height: f32, width: f32) -> Self {
        Self { height, width }
    }
}
