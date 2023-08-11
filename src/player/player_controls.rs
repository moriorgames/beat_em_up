use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use crate::combat::event::Event;
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

    pub fn handle_input(&mut self, ctx: &mut Context) -> Vec<Event> {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        let mut actions: Vec<Event> = Vec::new();

        if gamepad_intention.move_left || keyboard_intention.move_left {
            actions.push(Event::MoveLeft);
        }
        if gamepad_intention.move_right || keyboard_intention.move_right {
            actions.push(Event::MoveRight);
        }
        if gamepad_intention.move_up || keyboard_intention.move_up {
            actions.push(Event::MoveUp);
        }
        if gamepad_intention.move_down || keyboard_intention.move_down {
            actions.push(Event::MoveDown);
        }
        if gamepad_intention.attack || keyboard_intention.attack {
            actions.push(Event::Attack);
        }
        if keyboard_intention.quit {
            ctx.request_quit();
        }

        actions
    }
}
