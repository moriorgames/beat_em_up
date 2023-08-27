use crate::player::level_up_controls::LevelUpIntention;
use crate::player::player_controls::PlayerIntention;
use gilrs::EventType::{ButtonChanged, ButtonReleased};
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

    pub fn level_up(&mut self) -> LevelUpIntention {
        self.left_pressed = false;
        self.right_pressed = false;
        self.up_pressed = false;
        self.down_pressed = false;
        self.attack_pressed = false;
        self.jump_pressed = false;
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                ButtonReleased(Button::DPadLeft, _) => {
                    self.left_pressed = true;
                }
                ButtonReleased(Button::DPadRight, _) => {
                    self.right_pressed = true;
                }
                ButtonReleased(Button::DPadUp, _) => {
                    self.up_pressed = true;
                }
                ButtonReleased(Button::DPadDown, _) => {
                    self.down_pressed = true;
                }
                ButtonReleased(Button::West, _) => {
                    self.attack_pressed = true;
                }
                ButtonReleased(Button::South, _) => {
                    self.attack_pressed = true;
                }
                _ => {}
            }
        }

        LevelUpIntention {
            move_left: self.left_pressed,
            move_right: self.right_pressed,
            move_up: self.up_pressed,
            move_down: self.down_pressed,
            confirm: self.attack_pressed,
            quit: false,
        }
    }
}
