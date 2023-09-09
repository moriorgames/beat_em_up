use crate::player::level_up_controls::LevelUpIntention;
use crate::player::player_controls::PlayerIntention;
use gilrs::EventType::{ButtonChanged, ButtonPressed};
use gilrs::{Button, Event, Gilrs};

pub struct GamePadController {
    gilrs: Gilrs,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    fast_attack_pressed: bool,
    slow_attack_pressed: bool,
    jump_pressed: bool,
    defense_pressed: bool,
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
            fast_attack_pressed: false,
            slow_attack_pressed: false,
            jump_pressed: false,
            defense_pressed: false,
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
                    self.fast_attack_pressed = value > 0.0;
                }
                ButtonChanged(Button::North, value, _) => {
                    self.slow_attack_pressed = value > 0.0;
                }
                ButtonChanged(Button::South, value, _) => {
                    self.jump_pressed = value > 0.0;
                }
                ButtonChanged(Button::East, value, _) => {
                    self.defense_pressed = value > 0.0;
                }
                _ => {}
            }
        }

        PlayerIntention {
            move_left: self.left_pressed,
            move_right: self.right_pressed,
            move_up: self.up_pressed,
            move_down: self.down_pressed,
            fast_attack: self.fast_attack_pressed,
            slow_attack: self.slow_attack_pressed,
            jump: self.jump_pressed,
            defense: self.defense_pressed,
        }
    }

    pub fn level_up(&mut self) -> LevelUpIntention {
        self.left_pressed = false;
        self.right_pressed = false;
        self.up_pressed = false;
        self.down_pressed = false;
        self.fast_attack_pressed = false;
        self.jump_pressed = false;
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                ButtonPressed(Button::DPadLeft, _) => {
                    self.left_pressed = true;
                }
                ButtonPressed(Button::DPadRight, _) => {
                    self.right_pressed = true;
                }
                ButtonPressed(Button::DPadUp, _) => {
                    self.up_pressed = true;
                }
                ButtonPressed(Button::DPadDown, _) => {
                    self.down_pressed = true;
                }
                ButtonPressed(Button::West, _) => {
                    self.fast_attack_pressed = true;
                }
                ButtonPressed(Button::South, _) => {
                    self.fast_attack_pressed = true;
                }
                _ => {}
            }
        }

        LevelUpIntention {
            move_left: self.left_pressed,
            move_right: self.right_pressed,
            move_up: self.up_pressed,
            move_down: self.down_pressed,
            confirm: self.fast_attack_pressed,
        }
    }
}
