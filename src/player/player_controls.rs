use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use crate::character::character::Character;
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
    pub jump: bool,
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

    pub fn handle_input(
        &mut self,
        ctx: &mut Context,
        player: &Character,
        turn: u128,
    ) -> Vec<Action> {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        let mut actions: Vec<Action> = Vec::new();

        if player.is_idle() {
            if gamepad_intention.attack || keyboard_intention.attack {
                actions.push(Action::Attacking {
                    id: self.player_id,
                    from: turn + 1,
                    to: turn + 22,
                });
            }
            if (gamepad_intention.move_up || keyboard_intention.move_up)
                && (gamepad_intention.move_left || keyboard_intention.move_left)
            {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::UpLeft,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::UpLeft,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if (gamepad_intention.move_up || keyboard_intention.move_up)
                && (gamepad_intention.move_right || keyboard_intention.move_right)
            {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::UpRight,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::UpRight,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if (gamepad_intention.move_down || keyboard_intention.move_down)
                && (gamepad_intention.move_left || keyboard_intention.move_left)
            {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::DownLeft,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::DownLeft,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if (gamepad_intention.move_down || keyboard_intention.move_down)
                && (gamepad_intention.move_right || keyboard_intention.move_right)
            {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::DownRight,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::DownRight,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if gamepad_intention.move_left || keyboard_intention.move_left {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::Left,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::Left,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if gamepad_intention.move_right || keyboard_intention.move_right {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::Right,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::Right,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if gamepad_intention.move_up || keyboard_intention.move_up {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::Up,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::Up,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
            if gamepad_intention.move_down || keyboard_intention.move_down {
                if gamepad_intention.jump || keyboard_intention.jump {
                    actions.push(Action::Jumping {
                        id: self.player_id,
                        direction: Direction::Down,
                        from: turn + 1,
                        to: turn + 17,
                    });
                } else {
                    actions.push(Action::Moving {
                        id: self.player_id,
                        direction: Direction::Down,
                        from: turn + 1,
                        to: turn + 4,
                    });
                }
            }
        }
        if keyboard_intention.quit {
            ctx.request_quit();
        }

        actions
    }
}
