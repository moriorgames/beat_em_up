use super::controller::gamepad_controller::GamePadController;
use super::controller::keyboard_controller::keyboard_controller;
use crate::character::character::Character;
use crate::character::character_types::Facing;
use crate::combat::action::Action;
use crate::combat::direction::Direction;
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
    pub jump: bool,
}

impl PlayerControls {
    pub fn new() -> Self {
        let gamepad_controller: GamePadController = GamePadController::new();
        PlayerControls { gamepad_controller }
    }

    pub fn handle_input(
        &mut self,
        ctx: &mut Context,
        player: &Character,
        turn: u128,
    ) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        let intention: PlayerIntention = self.get_intention(ctx);

        if intention.attack {
            actions.push(Action::FastAttack {
                id: player.id,
                from: turn + 2,
                to: turn + player.fast_attack_timer as u128,
            });
        }

        if player.is_back_jumping() && intention.attack {
            let counter_attack_direction: Direction = match player.facing {
                Facing::Left => Direction::Left,
                Facing::Right => Direction::Right,
            };
            actions.push(Action::CounterAttack {
                id: player.id,
                direction: counter_attack_direction,
                from: turn + 3,
                to: turn + player.fast_attack_timer as u128 + 1,
            });
        }

        let directions: [(Direction, bool); 8] = [
            (Direction::UpLeft, intention.move_up && intention.move_left),
            (
                Direction::UpRight,
                intention.move_up && intention.move_right,
            ),
            (
                Direction::DownLeft,
                intention.move_down && intention.move_left,
            ),
            (
                Direction::DownRight,
                intention.move_down && intention.move_right,
            ),
            (Direction::Left, intention.move_left),
            (Direction::Right, intention.move_right),
            (Direction::Up, intention.move_up),
            (Direction::Down, intention.move_down),
        ];

        for &(direction, condition) in directions.iter() {
            if condition {
                let action = if intention.jump {
                    Action::Jump {
                        id: player.id,
                        direction,
                        from: turn + 1,
                        to: turn + 17,
                    }
                } else {
                    Action::Move {
                        id: player.id,
                        direction,
                        from: turn + 1,
                        to: turn + 6,
                    }
                };
                actions.push(action);
            }
        }

        if intention.jump
            && !intention.move_up
            && !intention.move_down
            && !intention.move_left
            && !intention.move_right
        {
            let opposite_direction = match player.facing {
                Facing::Left => Direction::Right,
                Facing::Right => Direction::Left,
            };

            actions.push(Action::BackJump {
                id: player.id,
                direction: opposite_direction,
                from: turn + 1,
                to: turn + 17,
            });
        }

        actions
    }

    fn get_intention(&mut self, ctx: &mut Context) -> PlayerIntention {
        let keyboard_intention: PlayerIntention = keyboard_controller::input(ctx);
        let gamepad_intention: PlayerIntention = self.gamepad_controller.input();

        PlayerIntention {
            move_left: keyboard_intention.move_left || gamepad_intention.move_left,
            move_right: keyboard_intention.move_right || gamepad_intention.move_right,
            move_up: keyboard_intention.move_up || gamepad_intention.move_up,
            move_down: keyboard_intention.move_down || gamepad_intention.move_down,
            attack: keyboard_intention.attack || gamepad_intention.attack,
            jump: keyboard_intention.jump || gamepad_intention.jump,
        }
    }
}
