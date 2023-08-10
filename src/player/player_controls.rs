use super::player_state::PlayerState;
use ggez::{input::keyboard::KeyCode, Context};
use gilrs::EventType::ButtonChanged;
use gilrs::{Button, Event, Gilrs};

pub struct PlayerControls {
    gilrs: Gilrs,
    dpad_left_pressed: bool,
    dpad_right_pressed: bool,
    dpad_up_pressed: bool,
    dpad_down_pressed: bool,
}

impl PlayerControls {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        PlayerControls {
            gilrs,
            dpad_left_pressed: false,
            dpad_right_pressed: false,
            dpad_up_pressed: false,
            dpad_down_pressed: false,
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context, player_state: &mut PlayerState) {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                ButtonChanged(Button::DPadLeft, value, _) => {
                    self.dpad_left_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadRight, value, _) => {
                    self.dpad_right_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadUp, value, _) => {
                    self.dpad_up_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadDown, value, _) => {
                    self.dpad_down_pressed = value > 0.0;
                }
                _ => {}
            }
        }

        if self.dpad_left_pressed {
            player_state.move_left();
        }
        if self.dpad_right_pressed {
            player_state.move_right();
        }
        if self.dpad_up_pressed {
            player_state.move_up();
        }
        if self.dpad_down_pressed {
            player_state.move_down();
        }

        let pressed_keys: &std::collections::HashSet<KeyCode> = ctx.keyboard.pressed_keys();
        if pressed_keys.contains(&KeyCode::Left) {
            player_state.move_left();
        }
        if pressed_keys.contains(&KeyCode::Right) {
            player_state.move_right();
        }
        if pressed_keys.contains(&KeyCode::Up) {
            player_state.move_up();
        }
        if pressed_keys.contains(&KeyCode::Down) {
            player_state.move_down();
        }
        if pressed_keys.contains(&KeyCode::Escape) {
            ctx.request_quit();
        }
    }
}
