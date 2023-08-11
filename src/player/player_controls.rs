use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use super::player_state::PlayerState;
use ggez::Context;

pub struct PlayerControls {
    gamepad_controller: GamePadController,
}

pub struct PlayerIntention {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub attack: bool,
    pub quit: bool,
}

impl PlayerControls {
    pub fn new() -> Self {
        let gamepad_controller: GamePadController = GamePadController::new();
        PlayerControls { gamepad_controller }
    }

    pub fn handle_input(&mut self, ctx: &mut Context, player_state: &mut PlayerState) {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        if gamepad_intention.move_left || keyboard_intention.move_left {
            player_state.move_left();
        }
        if gamepad_intention.move_right || keyboard_intention.move_right {
            player_state.move_right();
        }
        if gamepad_intention.move_up || keyboard_intention.move_up {
            player_state.move_up();
        }
        if gamepad_intention.move_down || keyboard_intention.move_down {
            player_state.move_down();
        }
        if gamepad_intention.attack || keyboard_intention.attack {
            player_state.attack();
        }
        if keyboard_intention.quit {
            ctx.request_quit();
        }
    }
}
