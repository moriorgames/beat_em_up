use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use ggez::Context;

pub struct LevelUpControls {
    gamepad_controller: GamePadController,
}

#[derive(Debug, Clone)]
pub struct LevelUpIntention {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub confirm: bool,
    pub quit: bool,
}

impl LevelUpControls {
    pub fn new() -> Self {
        let gamepad_controller: GamePadController = GamePadController::new();
        LevelUpControls { gamepad_controller }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) -> LevelUpIntention {
        let keyboard_intention: LevelUpIntention = keyboard_controller::level_up(ctx);
        let gamepad_intention: LevelUpIntention = self.gamepad_controller.level_up();

        let mut move_left: bool = false;
        let mut move_right: bool = false;
        let mut move_up: bool = false;
        let mut move_down: bool = false;
        let mut confirm: bool = false;
        let mut quit: bool = false;

        if keyboard_intention.move_left || gamepad_intention.move_left {
            move_left = true;
        }
        if keyboard_intention.move_right || gamepad_intention.move_right {
            move_right = true;
        }
        if keyboard_intention.move_up || gamepad_intention.move_up {
            move_up = true;
        }
        if keyboard_intention.move_down || gamepad_intention.move_down {
            move_down = true;
        }
        if keyboard_intention.confirm || gamepad_intention.confirm {
            confirm = true;
        }
        if keyboard_intention.quit || gamepad_intention.quit {
            quit = true;
        }

        LevelUpIntention {
            move_left,
            move_right,
            move_up,
            move_down,
            confirm,
            quit,
        }
    }
}
