#[derive(Clone)]
pub struct Size {
    pub h: f32,
    pub w: f32,
}

impl Size {
    pub fn new(height: f32, width: f32) -> Self {
        Self {
            h: height,
            w: width,
        }
    }
}
