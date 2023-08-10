use ggez::{input::keyboard::KeyCode, Context};
use gilrs::{Gilrs, Event, Button};
use gilrs::EventType::ButtonChanged;
use super::player_state::PlayerState;

pub struct PlayerControls {
    gilrs: Gilrs,
    dpad_right_pressed: bool,
}

impl PlayerControls {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        PlayerControls {
            gilrs,
            dpad_right_pressed: false,
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context, player_state: &mut PlayerState) {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                ButtonChanged(Button::DPadRight, value, _) => {
                    self.dpad_right_pressed = value > 0.0;
                },
                _ => {}
            }
        }

        if self.dpad_right_pressed {
            player_state.move_right();
        }

        let pressed_keys: &std::collections::HashSet<KeyCode> = ctx.keyboard.pressed_keys();
        if pressed_keys.contains(&KeyCode::Right) {
            player_state.move_right();
        }
    }    
}
