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
    pub animation_frame: u8,
}

impl Character {
    pub fn new(
        position: Position,
        size: Size,
        speed: f32,
        max_health: f32,
        character_type: CharacterTypes,
    ) -> Self {
        Character {
            id: Uuid::new_v4(),
            position,
            size,
            speed,
            current_health: max_health,
            max_health,
            character_type,
            animation_frame: 0,
        }
    }

    pub fn move_by_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
    }

    fn move_left(&mut self) {
        self.position.x -= self.speed;
        self.update_move_animation();
    }

    fn move_right(&mut self) {
        self.position.x += self.speed;
        self.update_move_animation();
    }

    fn move_up(&mut self) {
        self.position.y -= self.speed;
        self.update_move_animation();
    }

    fn move_down(&mut self) {
        self.position.y += self.speed;
        self.update_move_animation();
    }

    fn update_move_animation(&mut self) {
        self.animation_frame += 1;
    }
}
