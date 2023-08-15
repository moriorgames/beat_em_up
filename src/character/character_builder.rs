pub mod character_builder {
    use crate::{
        character::{character::Character, character_types::CharacterTypes},
        geometry::{position::Position, size::Size},
    };

    pub fn build() -> Vec<Character> {
        let mut characters: Vec<Character> = Vec::new();

        // Player
        let position: Position = Position::new(400.0, 300.0);
        let size: Size = Size::new(55.0, 45.0);
        let speed: f32 = 3.7;
        let max_health: f32 = 1000.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let player: Character = Character::new(position, size, speed, max_health, character_type);
        characters.push(player);

        // Enemy
        let position: Position = Position::new(600.0, 400.0);
        let size: Size = Size::new(45.0, 35.0);
        let speed: f32 = 1.3;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type);
        characters.push(enemy);

        // Enemy
        let position: Position = Position::new(700.0, 800.0);
        let size: Size = Size::new(45.0, 35.0);
        let speed: f32 = 1.3;
        let max_health: f32 = 800.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let enemy: Character = Character::new(position, size, speed, max_health, character_type);
        characters.push(enemy);

        characters
    }
}
