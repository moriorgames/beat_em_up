use super::animation::Animation;
use super::box_collision::BoxCollision;
use super::character_types::{CharacterTypes, Facing};
use crate::combat::direction::Direction;
use crate::combat::event::Event;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use uuid::Uuid;

#[derive(Clone)]
pub struct Character {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub speed: f32,
    pub current_health: f32,
    pub max_health: f32,
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
    Moving,
    Attacking,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        max_health: f32,
        character_type: CharacterTypes,
        move_animation: Animation,
        attack_animation: Animation,
        body_collision: BoxCollision,
        foot_collision: BoxCollision,
    ) -> Self {
        Character {
            id: Uuid::new_v4(),
            position,
            size,
            speed,
            current_health: max_health,
            max_health,
            character_type,
            facing: Facing::Right,
            move_animation,
            attack_animation,
            attack_timer: 0,
            body_collision,
            foot_collision,
            weapon_collision: BoxCollision {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            character_state: CharacterState::Moving,
        }
    }

    pub fn update(&mut self) -> Option<Event> {
        match self.character_state {
            CharacterState::Moving => {
                self.move_animation.update();

                None
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
                if self.attack_timer <= 0 {
                    self.attack_timer = 0;
                    self.character_state = CharacterState::Moving;
                    self.attack_animation.moved = false;
                    self.attack_animation.counter = 0;
                    self.attack_animation.frame = 0;
                    self.weapon_collision = BoxCollision {
                        x: 0.0,
                        y: 0.0,
                        w: 0.0,
                        h: 0.0,
                    };

                    None
                } else {
                    Some(Event::Attack { id: self.id })
                }
            }
        }
    }

    pub fn move_by_direction(&mut self, direction: Direction) {
        if self.character_state == CharacterState::Moving {
            match direction {
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
            }
        }
    }

    pub fn attack(&mut self) {
        self.attack_timer = 24;
        self.character_state = CharacterState::Attacking;
    }

    pub fn apply_damage(&mut self, damage: f32) {
        self.current_health -= damage;
    }

    pub fn get_sprite_name(&self) -> String {
        match self.character_state {
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

    pub fn next_foot_collision_to_world_space(&self, direction: Direction) -> BoxCollision {
        let mut x: f32 = self.position.x;
        let mut y: f32 = self.position.y;
        match direction {
            Direction::Left => x -= self.speed,
            Direction::Right => x += self.speed,
            Direction::Up => y -= self.speed,
            Direction::Down => y += self.speed,
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
