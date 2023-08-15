pub mod character_builder {
    use crate::{
        character::{character::Character, character_types::CharacterTypes},
        geometry::{position::Position, size::Size},
    };

    pub fn build() -> Vec<Character> {
        let mut characters: Vec<Character> = Vec::new();

        // Player
        let position: Position = Position::new(500.0, 500.0);
        let size: Size = Size::new(128.0, 128.0);
        let speed: f32 = 4.7;
        let max_health: f32 = 1000.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let sprite: String = "barbarian".to_string();
        let move_frames: u8 = 8;
        let player: Character = Character::new(position, size, speed, max_health, character_type, sprite, move_frames);
        characters.push(player);

        // Enemy
        let position: Position = Position::new(100.0, 100.0);
        let size: Size = Size::new(128.0, 128.0);
        let speed: f32 = 2.3;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let sprite: String = "orc".to_string();
        let move_frames: u8 = 6;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type, sprite, move_frames);
        characters.push(enemy);

        // Enemy
        let position: Position = Position::new(1000.0, 1000.0);
        let size: Size = Size::new(128.0, 128.0);
        let speed: f32 = 2.9;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let sprite: String = "orc".to_string();
        let move_frames: u8 = 6;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type, sprite, move_frames);
        characters.push(enemy);

        characters
    }
}
