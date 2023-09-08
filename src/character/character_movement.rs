use super::{character::Character, character_state::CharacterState, character_types::Facing};
use crate::combat::direction::Direction;

impl Character {
    const DIAGONAL_SPEED_FACTOR: f32 = 0.8;

    pub fn jump_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Jumping
            || self.character_state == CharacterState::BackJumping
            || self.character_state == CharacterState::CounterAttacking
        {
            self.action_processed = true;
            match direction {
                Direction::Left => self.move_left(self.speed_jump),
                Direction::Right => self.move_right(self.speed_jump),
                Direction::Up => self.move_up(self.speed_jump),
                Direction::Down => self.move_down(self.speed_jump),
                Direction::UpLeft => {
                    self.move_up(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_left(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::UpRight => {
                    self.move_up(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_right(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::DownLeft => {
                    self.move_down(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_left(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::DownRight => {
                    self.move_down(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_right(self.speed_jump * Self::DIAGONAL_SPEED_FACTOR);
                }
            }

            if self.character_state == CharacterState::BackJumping {
                if self.facing == Facing::Left {
                    self.facing = Facing::Right;
                } else {
                    self.facing = Facing::Left;
                }
            }
        }
    }

    pub fn move_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Moving {
            self.action_processed = true;
            match direction {
                Direction::Left => self.move_left(self.speed),
                Direction::Right => self.move_right(self.speed),
                Direction::Up => self.move_up(self.speed),
                Direction::Down => self.move_down(self.speed),
                Direction::UpLeft => {
                    self.move_up(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_left(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::UpRight => {
                    self.move_up(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_right(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::DownLeft => {
                    self.move_down(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_left(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                }
                Direction::DownRight => {
                    self.move_down(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                    self.move_right(self.speed * Self::DIAGONAL_SPEED_FACTOR);
                }
            }
        }
    }

    fn move_left(&mut self, speed: f32) {
        self.position.x -= speed;
        if self.facing == Facing::Right {
            self.body_collision.x *= -1.0;
        }
        self.facing = Facing::Left;
    }

    fn move_right(&mut self, speed: f32) {
        self.position.x += speed;
        if self.facing == Facing::Left {
            self.body_collision.x *= -1.0;
        }
        self.facing = Facing::Right;
    }

    fn move_up(&mut self, speed: f32) {
        self.position.y -= speed;
    }

    fn move_down(&mut self, speed: f32) {
        self.position.y += speed;
    }
}
