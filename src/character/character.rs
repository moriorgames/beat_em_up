use super::character_types::CharacterTypes;
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
    // Animation
    pub sprite: String,
    pub move_frames: u8,
    pub animation_frame: u8,
    pub animation_moved: bool,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        max_health: f32,
        character_type: CharacterTypes,
        sprite: String,
        move_frames: u8,
    ) -> Self {
        Character {
            id: Uuid::new_v4(),
            position,
            size,
            speed,
            current_health: max_health,
            max_health,
            character_type,
            sprite,
            move_frames,
            animation_frame: 0,
            animation_moved: false,
        }
    }

    pub fn update(&mut self) {
        self.animation_moved = false;
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
        let animation_frame: u8 = self.animation_frame % self.move_frames;

        format!("{}_{}_{}", self.sprite, movement_type, animation_frame)
    }

    fn move_left(&mut self) {
        self.position.x -= self.speed;
    }

    fn move_right(&mut self) {
        self.position.x += self.speed;
    }

    fn move_up(&mut self) {
        self.position.y -= self.speed;
    }

    fn move_down(&mut self) {
        self.position.y += self.speed;
    }

    pub fn update_move_animation(&mut self) {
        if !self.animation_moved {
            self.animation_frame += 1;
            if self.animation_frame > 80 {
                self.animation_frame = 0;
            }
            self.animation_moved = true;
        }
    }
}
