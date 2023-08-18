use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use crate::combat::action::Action;
use crate::combat::direction::Direction;
use ggez::Context;
use uuid::Uuid;

pub struct PlayerControls {
    player_id: Uuid,
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
    pub fn new(player_id: Uuid) -> Self {
        let gamepad_controller: GamePadController = GamePadController::new();
        PlayerControls {
            player_id,
            gamepad_controller,
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context, turn: u128) -> Vec<Action> {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        let mut actions: Vec<Action> = Vec::new();

        if gamepad_intention.attack || keyboard_intention.attack {
            actions.push(Action::StartAttacking {
                id: self.player_id,
                from: turn,
                to: turn + 2,
            });
        }
        if (gamepad_intention.move_up || keyboard_intention.move_up)
            && (gamepad_intention.move_left || keyboard_intention.move_left)
        {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::UpLeft,
                from: turn,
                to: turn + 2,
            });
        }
        if (gamepad_intention.move_up || keyboard_intention.move_up)
            && (gamepad_intention.move_right || keyboard_intention.move_right)
        {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::UpRight,
                from: turn,
                to: turn + 2,
            });
        }
        if (gamepad_intention.move_down || keyboard_intention.move_down)
            && (gamepad_intention.move_left || keyboard_intention.move_left)
        {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::DownLeft,
                from: turn,
                to: turn + 2,
            });
        }
        if (gamepad_intention.move_down || keyboard_intention.move_down)
            && (gamepad_intention.move_right || keyboard_intention.move_right)
        {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::DownRight,
                from: turn,
                to: turn + 2,
            });
        }
        if gamepad_intention.move_left || keyboard_intention.move_left {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::Left,
                from: turn,
                to: turn + 2,
            });
        }
        if gamepad_intention.move_right || keyboard_intention.move_right {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::Right,
                from: turn,
                to: turn + 2,
            });
        }
        if gamepad_intention.move_up || keyboard_intention.move_up {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::Up,
                from: turn,
                to: turn + 2,
            });
        }
        if gamepad_intention.move_down || keyboard_intention.move_down {
            actions.push(Action::StartMoving {
                id: self.player_id,
                direction: Direction::Down,
                from: turn,
                to: turn + 2,
            });
        }
        if keyboard_intention.quit {
            ctx.request_quit();
        }

        actions
    }
}
