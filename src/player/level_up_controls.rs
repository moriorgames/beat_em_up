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
}

impl LevelUpControls {
    pub fn new() -> Self {
        let gamepad_controller: GamePadController = GamePadController::new();
        LevelUpControls { gamepad_controller }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) -> LevelUpIntention {
        self.get_level_up_intention(ctx)
    }

    fn get_level_up_intention(&mut self, ctx: &mut Context) -> LevelUpIntention {
        let keyboard_intention: LevelUpIntention = keyboard_controller::level_up(ctx);
        let gamepad_intention: LevelUpIntention = self.gamepad_controller.level_up();

        LevelUpIntention {
            move_left: keyboard_intention.move_left || gamepad_intention.move_left,
            move_right: keyboard_intention.move_right || gamepad_intention.move_right,
            move_up: keyboard_intention.move_up || gamepad_intention.move_up,
            move_down: keyboard_intention.move_down || gamepad_intention.move_down,
            confirm: keyboard_intention.confirm || gamepad_intention.confirm,
        }
    }
}
