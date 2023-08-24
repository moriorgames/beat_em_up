use super::animation::Animation;
use super::box_collision::BoxCollision;
use super::character_stats::Stats;
use super::character_types::{CharacterTypes, Facing};
use crate::combat::direction::Direction;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Character {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub stats: Stats,
    pub speed: f32,
    pub speed_jump: f32,
    pub damage: f32,
    pub defense: f32,
    pub current_health: f32,
    pub health: f32,
    pub has_processed_action: bool,
    pub character_type: CharacterTypes,
    pub facing: Facing,
    pub move_animation: Animation,
    pub attack_animation: Animation,
    pub jump_animation: Animation,
    pub full_attack_timer: u8,
    pub attack_timer: u8,
    pub attack_timer_hit: u8,
    pub body_collision: BoxCollision,
    pub foot_collision: BoxCollision,
    pub weapon_collision: BoxCollision,
    pub character_state: CharacterState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterState {
    Idle,
    Moving,
    Jumping,
    Attacking,
    Damaged,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        stats: Stats,
        character_type: CharacterTypes,
        move_animation: Animation,
        attack_animation: Animation,
        jump_animation: Animation,
        body_collision: BoxCollision,
        foot_collision: BoxCollision,
    ) -> Self {
        let full_attack_timer: u8 = attack_animation.move_frames * attack_animation.delay;
        let attack_timer_hit: u8 = full_attack_timer / 3 + 2;
        let (speed, speed_jumping, damage, defense, health) = stats.get_calculated_stats();
        Character {
            id: Uuid::new_v4(),
            position,
            size,
            stats,
            speed,
            speed_jump: speed * 1.5,
            damage,
            defense,
            current_health: health,
            health,
            has_processed_action: false,
            character_type,
            facing: Facing::Right,
            move_animation,
            attack_animation,
            jump_animation,
            full_attack_timer,
            attack_timer: 0,
            attack_timer_hit,
            body_collision,
            foot_collision,
            weapon_collision: BoxCollision {
                x: 2000.0,
                y: 2000.0,
                w: 0.0,
                h: 0.0,
            },
            character_state: CharacterState::Idle,
        }
    }

    pub fn is_idle(&self) -> bool {
        self.character_state == CharacterState::Idle
    }

    pub fn is_moving(&self) -> bool {
        self.character_state == CharacterState::Moving
    }

    pub fn is_jumping(&self) -> bool {
        self.character_state == CharacterState::Jumping
    }

    pub fn is_attacking(&self) -> bool {
        self.character_state == CharacterState::Attacking
    }

    pub fn is_damaged(&self) -> bool {
        self.character_state == CharacterState::Damaged
    }

    pub fn start_moving(&mut self) {
        self.character_state = CharacterState::Moving
    }

    pub fn start_jumping(&mut self) {
        self.character_state = CharacterState::Jumping
    }

    pub fn start_attacking(&mut self) {
        self.attack_timer = self.full_attack_timer;
        self.character_state = CharacterState::Attacking
    }

    pub fn start_damaged(&mut self, damage: f32) {
        self.character_state = CharacterState::Damaged;
        self.apply_damage(damage);
    }

    pub fn back_to_idle(&mut self) {
        self.attack_animation.reset();
        self.jump_animation.reset();
        self.character_state = CharacterState::Idle;
        self.attack_timer = 0;
        self.weapon_collision = BoxCollision {
            x: 2000.0,
            y: 2000.0,
            w: 0.0,
            h: 0.0,
        };
    }

    pub fn update(&mut self) {
        self.has_processed_action = false;
        match self.character_state {
            CharacterState::Idle => {
                self.move_animation.update();
            }
            CharacterState::Moving => {
                self.move_animation.update();
            }
            CharacterState::Jumping => {
                self.jump_animation.update();
            }
            CharacterState::Attacking => {
                self.attack_animation.update();
                self.attack_timer -= 1;
                if self.attack_timer <= self.attack_timer_hit {
                    let x: f32 = match self.facing {
                        Facing::Left => -75.0,
                        Facing::Right => 75.0,
                    };
                    self.weapon_collision = BoxCollision {
                        x,
                        y: -85.0,
                        w: 75.0,
                        h: 60.0,
                    };
                }
            }
            CharacterState::Damaged => {
                self.move_animation.update();
            }
        }
    }

    pub fn attack(&mut self) {
        if self.character_state == CharacterState::Attacking {
            self.has_processed_action = true;
        }
    }

    pub fn jump_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Jumping {
            self.has_processed_action = true;
            match direction {
                Direction::Left => self.move_left(self.speed_jump),
                Direction::Right => self.move_right(self.speed_jump),
                Direction::Up => self.move_up(self.speed_jump),
                Direction::Down => self.move_down(self.speed_jump),
                Direction::UpLeft => {
                    self.move_up(self.speed_jump);
                    self.move_left(self.speed_jump);
                }
                Direction::UpRight => {
                    self.move_up(self.speed_jump);
                    self.move_right(self.speed_jump);
                }
                Direction::DownLeft => {
                    self.move_down(self.speed_jump);
                    self.move_left(self.speed_jump);
                }
                Direction::DownRight => {
                    self.move_down(self.speed_jump);
                    self.move_right(self.speed_jump);
                }
            }
        }
    }

    pub fn move_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Moving {
            self.has_processed_action = true;
            match direction {
                Direction::Left => self.move_left(self.speed),
                Direction::Right => self.move_right(self.speed),
                Direction::Up => self.move_up(self.speed),
                Direction::Down => self.move_down(self.speed),
                Direction::UpLeft => {
                    self.move_up(self.speed);
                    self.move_left(self.speed);
                }
                Direction::UpRight => {
                    self.move_up(self.speed);
                    self.move_right(self.speed);
                }
                Direction::DownLeft => {
                    self.move_down(self.speed);
                    self.move_left(self.speed);
                }
                Direction::DownRight => {
                    self.move_down(self.speed);
                    self.move_right(self.speed);
                }
            }
        }
    }

    fn move_left(&mut self, speed: f32) {
        self.position.x -= speed;
        self.facing = Facing::Left;
    }

    fn move_right(&mut self, speed: f32) {
        self.position.x += speed;
        self.facing = Facing::Right;
    }

    fn move_up(&mut self, speed: f32) {
        self.position.y -= speed;
    }

    fn move_down(&mut self, speed: f32) {
        self.position.y += speed;
    }

    fn apply_damage(&mut self, damage: f32) {
        self.current_health -= damage;
        if self.current_health <= 0.0 {
            self.current_health = 0.0;
        }
        let push_x: f32 = match self.facing {
            Facing::Left => 8.0,
            Facing::Right => -8.0,
        };
        self.position.x += push_x;
    }

    pub fn get_sprite_name(&self) -> String {
        match self.character_state {
            CharacterState::Idle => {
                let animation_frame: u8 =
                    self.move_animation.frame % self.move_animation.move_frames;
                let action_type: String = self.move_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.move_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Moving => {
                let animation_frame: u8 =
                    self.move_animation.frame % self.move_animation.move_frames;
                let action_type: String = self.move_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.move_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Jumping => {
                let animation_frame: u8 =
                    self.jump_animation.frame % self.jump_animation.move_frames;
                let action_type: String = self.jump_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.jump_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Attacking => {
                let animation_frame: u8 =
                    self.attack_animation.frame % self.attack_animation.move_frames;
                let action_type: String = self.attack_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.attack_animation.sprite, action_type, animation_frame
                )
            }
            CharacterState::Damaged => {
                let animation_frame: u8 =
                    self.move_animation.frame % self.move_animation.move_frames;
                let action_type: String = self.move_animation.action_type.to_string();

                format!(
                    "{}_{}_{}",
                    self.move_animation.sprite, action_type, animation_frame
                )
            }
        }
    }

    pub fn next_foot_collision_to_world_space(&self, direction: Direction) -> BoxCollision {
        let mut x: f32 = self.position.x;
        let mut y: f32 = self.position.y;
        match direction {
            Direction::Left => x -= self.speed,
            Direction::Right => x += self.speed,
            Direction::Up => y -= self.speed,
            Direction::Down => y += self.speed,
            Direction::UpLeft => {
                y -= self.speed;
                x -= self.speed;
            }
            Direction::UpRight => {
                y -= self.speed;
                x += self.speed;
            }
            Direction::DownLeft => {
                y += self.speed;
                x -= self.speed;
            }
            Direction::DownRight => {
                y += self.speed;
                x += self.speed;
            }
        }

        let position: Position = Position::new(x, y);
        BoxCollision {
            x: position.x + self.foot_collision.x - self.foot_collision.w / 2.0,
            y: position.y + self.foot_collision.y - self.foot_collision.y / 2.0,
            w: self.foot_collision.w,
            h: self.foot_collision.h,
        }
    }

    pub fn body_collision_to_world_space(&self) -> BoxCollision {
        self.body_collision.to_world_space(self.position.clone())
    }

    pub fn foot_collision_to_world_space(&self) -> BoxCollision {
        self.foot_collision.to_world_space(self.position.clone())
    }

    pub fn weapon_collision_to_world_space(&self) -> BoxCollision {
        self.weapon_collision.to_world_space(self.position.clone())
    }
}
