#[derive(Clone)]
pub struct Animation {
    pub sprite: String,
    pub action_type: String,
    pub move_frames: u8,
    pub frame: u8,
    pub delay: u8,
    pub counter: u8,
    pub moved: bool,
}

impl Animation {
    pub fn new(sprite: String, action_type: String, move_frames: u8, delay: u8) -> Self {
        Self {
            sprite,
            action_type,
            move_frames,
            frame: 0,
            delay,
            counter: 0,
            moved: false,
        }
    }
}
