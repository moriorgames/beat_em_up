use crate::player::player_controls::PlayerIntention;
use gilrs::EventType::ButtonChanged;
use gilrs::{Button, Event, Gilrs};

pub struct GamePadController {
    gilrs: Gilrs,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    attack_pressed: bool,
    jump_pressed: bool,
}

impl GamePadController {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        GamePadController {
            gilrs,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            attack_pressed: false,
            jump_pressed: false,
        }
    }

    pub fn input(&mut self) -> PlayerIntention {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                ButtonChanged(Button::DPadLeft, value, _) => {
                    self.left_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadRight, value, _) => {
                    self.right_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadUp, value, _) => {
                    self.up_pressed = value > 0.0;
                }
                ButtonChanged(Button::DPadDown, value, _) => {
                    self.down_pressed = value > 0.0;
                }
                ButtonChanged(Button::West, value, _) => {
                    self.attack_pressed = value > 0.0;
                }
                ButtonChanged(Button::South, value, _) => {
                    self.jump_pressed = value > 0.0;
                }
                _ => {}
            }
        }

        PlayerIntention {
            move_left: self.left_pressed,
            move_right: self.right_pressed,
            move_up: self.up_pressed,
            move_down: self.down_pressed,
            attack: self.attack_pressed,
            jump: self.jump_pressed,
            quit: false,
        }
    }
}
