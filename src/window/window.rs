use ggez::conf::{WindowMode, WindowSetup};

pub struct Window;

impl Window {
    pub const WIDTH: f32 = 1920.0;
    pub const HEIGHT: f32 = 1080.0;
    pub const TITLE: &str = "Beat 'em up";

    pub fn create_window_setup() -> WindowSetup {
        WindowSetup::default().title(Self::TITLE)
    }

    pub fn create_window_mode() -> WindowMode {
        WindowMode::default().dimensions(Self::WIDTH, Self::HEIGHT)
    }
}
