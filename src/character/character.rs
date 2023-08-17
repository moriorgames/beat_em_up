use super::animation::Animation;
use super::character_types::{CharacterTypes, Facing};
use super::collision::Box;
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
    pub foot_collision: Box,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        max_health: f32,
        character_type: CharacterTypes,
        animation: Animation,
        foot_collision: Box,
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
        let movement_type: &str = "move";
        let animation_frame: u8 = self.animation.frame % self.animation.move_frames;

        format!(
            "{}_{}_{}",
            self.animation.sprite, movement_type, animation_frame
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

    pub fn foot_collision_to_world_space(&self) -> Box {
        Box {
            x: self.position.x + self.foot_collision.x,
            y: self.position.y + self.foot_collision.y,
            w: self.foot_collision.w,
            h: self.foot_collision.h,
        }
    }
}
