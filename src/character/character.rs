use super::animation::Animation;
use super::box_collision::BoxCollision;
use super::character_types::{CharacterTypes, Facing};
use crate::combat::direction::Direction;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

#[derive(Clone)]
pub struct Character {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub speed: f32,
    pub strength: f32,
    pub armor: f32,
    pub current_health: f32,
    pub max_health: f32,
    pub has_processed_action: bool,
    pub character_type: CharacterTypes,
    pub facing: Facing,
    pub move_animation: Animation,
    pub attack_animation: Animation,
    pub attack_timer: u8,
    pub body_collision: BoxCollision,
    pub foot_collision: BoxCollision,
    pub weapon_collision: BoxCollision,
    pub character_state: CharacterState,
}

#[derive(Clone, PartialEq)]
pub enum CharacterState {
    Idle,
    Moving,
    Attacking,
    Damaged,
}

impl Character {
    pub fn new(
        id: Uuid,
        position: Position,
        size: Size,
        speed: f32,
        strength: f32,
        armor: f32,
        max_health: f32,
        character_type: CharacterTypes,
        move_animation: Animation,
        attack_animation: Animation,
        body_collision: BoxCollision,
        foot_collision: BoxCollision,
    ) -> Self {
        Character {
            id,
            position,
            size,
            speed,
            strength,
            armor,
            current_health: max_health,
            max_health,
            has_processed_action: false,
            character_type,
            facing: Facing::Right,
            move_animation,
            attack_animation,
            attack_timer: 0,
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

    pub fn is_idle(&mut self) -> bool {
        self.character_state == CharacterState::Idle
    }

    pub fn is_moving(&mut self) -> bool {
        self.character_state == CharacterState::Moving
    }

    pub fn is_attacking(&mut self) -> bool {
        self.character_state == CharacterState::Attacking
    }
    
    pub fn is_damaged(&mut self) -> bool {
        self.character_state == CharacterState::Damaged
    }

    pub fn start_moving(&mut self) {
        self.character_state = CharacterState::Moving
    }

    pub fn start_attacking(&mut self) {
        self.attack_timer = 24;
        self.character_state = CharacterState::Attacking
    }

    pub fn start_damaged(&mut self, damage: f32) {
        self.character_state = CharacterState::Damaged;
        self.apply_damage(damage);
    }
    
    pub fn back_to_idle(&mut self) {
        self.attack_animation.reset();
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
            CharacterState::Attacking => {
                self.attack_animation.update();
                self.attack_timer -= 1;
                if self.attack_timer <= 12 {
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

    pub fn move_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Moving {
            self.has_processed_action = true;
            match direction {
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
                Direction::UpLeft => {
                    self.move_up();
                    self.move_left();
                }
                Direction::UpRight => {
                    self.move_up();
                    self.move_right();
                }
                Direction::DownLeft => {
                    self.move_down();
                    self.move_left();
                }
                Direction::DownRight => {
                    self.move_down();
                    self.move_right();
                }
            }
        }
    }

    fn move_left(&mut self) {
        self.position.x -= self.speed;
        self.facing = Facing::Left;
    }

    fn move_right(&mut self) {
        self.position.x += self.speed;
        self.facing = Facing::Right;
    }

    fn move_up(&mut self) {
        self.position.y -= self.speed;
    }

    fn move_down(&mut self) {
        self.position.y += self.speed;
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
