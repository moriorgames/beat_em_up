use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use crate::combat::event::Event;
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

    pub fn handle_input(&mut self, ctx: &mut Context) -> Vec<Event> {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        let mut events: Vec<Event> = Vec::new();

        if gamepad_intention.move_left || keyboard_intention.move_left {
            events.push(Event::MoveLeft { id: self.player_id });
        }
        if gamepad_intention.move_right || keyboard_intention.move_right {
            events.push(Event::MoveRight { id: self.player_id });
        }
        if gamepad_intention.move_up || keyboard_intention.move_up {
            events.push(Event::MoveUp { id: self.player_id });
        }
        if gamepad_intention.move_down || keyboard_intention.move_down {
            events.push(Event::MoveDown { id: self.player_id });
        }
        if gamepad_intention.attack || keyboard_intention.attack {
            events.push(Event::Attack { id: self.player_id });
        }
        if keyboard_intention.quit {
            ctx.request_quit();
        }

        events
    }
}
