pub mod character_builder {
    use crate::{
        character::{character::Character, character_types::CharacterTypes},
        geometry::{position::Position, size::Size},
    };

    pub fn build() -> Vec<Character> {
        let mut characters: Vec<Character> = Vec::new();

        // Player
        let position: Position = Position::new(600.0, 700.0);
        let size: Size = Size::new(120.0, 120.0);
        let speed: f32 = 4.3;
        let max_health: f32 = 1000.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let sprite: String = "barbarian".to_string();
        let move_frames: u8 = 8;
        let animation_delay: u8 = 4;
        let player: Character = Character::new(
            position, size, speed, max_health, character_type, sprite, move_frames, animation_delay
        );
        characters.push(player);

        // Enemy
        let position: Position = Position::new(100.0, 600.0);
        let size: Size = Size::new(120.0, 120.0);
        let speed: f32 = 2.1;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let sprite: String = "orc".to_string();
        let move_frames: u8 = 6;
        let animation_delay: u8 = 6;
        let enemy: Character = Character::new(
            position, size, speed, max_health, character_type, sprite, move_frames, animation_delay
        );
        characters.push(enemy);

        // Enemy
        let position: Position = Position::new(1200.0, 800.0);
        let size: Size = Size::new(120.0, 120.0);
        let speed: f32 = 2.7;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let sprite: String = "orc".to_string();
        let move_frames: u8 = 6;
        let animation_delay: u8 = 6;
        let enemy: Character = Character::new(
            position, size, speed, max_health, character_type, sprite, move_frames, animation_delay
        );
        characters.push(enemy);

        characters
    }
}
