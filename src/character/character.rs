use super::animation::Animation;
use super::box_collision::{self, BoxCollision};
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
    pub current_health: f32,
    pub max_health: f32,
    pub character_type: CharacterTypes,
    pub facing: Facing,
    pub animation: Animation,
    pub body_collision: BoxCollision,
    pub foot_collision: BoxCollision,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        max_health: f32,
        character_type: CharacterTypes,
        animation: Animation,
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
            animation,
            body_collision,
            foot_collision,
        }
    }

    pub fn update(&mut self) {
        if self.animation.counter > self.animation.delay {
            self.animation.moved = false;
            self.animation.counter = 0;
        }
        self.animation.counter += 1;
    }

    pub fn move_by_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }

    pub fn get_sprite_name(&self) -> String {
        let animation_frame: u8 = self.animation.frame % self.animation.move_frames;

        format!(
            "{}_{}_{}",
            self.animation.sprite, self.animation.action_type, animation_frame
        )
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

    pub fn update_move_animation(&mut self) {
        if !self.animation.moved {
            self.animation.frame += 1;
            if self.animation.frame > 80 {
                self.animation.frame = 0;
            }
            self.animation.moved = true;
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
        self.box_collision_to_world_space(self.body_collision.clone())
    }

    pub fn foot_collision_to_world_space(&self) -> BoxCollision {
        self.box_collision_to_world_space(self.foot_collision.clone())
    }

    fn box_collision_to_world_space(&self, box_collision: BoxCollision) -> BoxCollision {
        BoxCollision {
            x: self.position.x + box_collision.x - box_collision.w / 2.0,
            y: self.position.y + box_collision.y - box_collision.y / 2.0,
            w: box_collision.w,
            h: box_collision.h,
        }
    }
}
