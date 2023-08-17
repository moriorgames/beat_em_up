pub mod character_builder {
    use crate::{
        character::{
            animation::Animation, box_collision::CollisionBox, character::Character,
            character_types::CharacterTypes,
        },
        geometry::{position::Position, size::Size},
    };

    pub fn build() -> Vec<Character> {
        let mut characters: Vec<Character> = Vec::new();

        let position: Position = Position::new(600.0, 700.0);
        let size: Size = Size::new(190.0, 190.0);
        let speed: f32 = 5.3;
        let max_health: f32 = 1000.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let animation: Animation = create_barbarian_animation();
        let foot_collision: CollisionBox = CollisionBox {
            position: Position::new(0.0, 50.0),
            size: Size::new(size.w - 30.0, 50.0),
        };
        let player: Character = Character::new(
            position,
            size,
            speed,
            max_health,
            character_type,
            animation,
            foot_collision,
        );
        characters.push(player);

        let position: Position = Position::new(1400.0, 850.0);
        let size: Size = Size::new(190.0, 190.0);
        let speed: f32 = 3.1;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let animation: Animation = create_orc_animation();
        let foot_collision: CollisionBox = CollisionBox {
            position: Position::new(0.0, 50.0),
            size: Size::new(size.w - 30.0, 50.0),
        };
        let enemy: Character = Character::new(
            position,
            size,
            speed,
            max_health,
            character_type,
            animation,
            foot_collision,
        );
        characters.push(enemy);

        characters
    }

    fn create_barbarian_animation() -> Animation {
        let sprite: String = "barbarian".to_string();
        let move_frames: u8 = 8;
        let delay: u8 = 4;

        Animation::new(sprite, move_frames, delay)
    }

    fn create_orc_animation() -> Animation {
        let sprite: String = "orc".to_string();
        let move_frames: u8 = 6;
        let delay: u8 = 6;

        Animation::new(sprite, move_frames, delay)
    }
}
