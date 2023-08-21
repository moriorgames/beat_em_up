pub mod character_builder {
    use crate::{
        character::{
            animation::Animation, box_collision::BoxCollision, character::Character,
            character_types::CharacterTypes,
        },
        geometry::{position::Position, size::Size},
    };

    pub fn build() -> Vec<Character> {
        let mut characters: Vec<Character> = Vec::new();

        let position: Position = Position::new(800.0, 700.0);
        let size: Size = Size::new(190.0, 190.0);
        let speed: f32 = 5.3;
        let strength: f32 = 21.0;
        let armor: f32 = 4.0;
        let max_health: f32 = 100.0;
        let character_type: CharacterTypes = CharacterTypes::Player;
        let move_animation: Animation = create_barbarian_move_animation();
        let attack_animation: Animation = create_barbarian_attack_animation();
        let body_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: -125.0,
            w: size.w - 75.0,
            h: 125.0,
        };
        let foot_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: 50.0,
            w: size.w - 30.0,
            h: 50.0,
        };
        let player: Character = Character::new(
            position,
            size,
            speed,
            strength,
            armor,
            max_health,
            character_type,
            move_animation,
            attack_animation,
            body_collision,
            foot_collision,
        );
        characters.push(player);

        let position: Position = Position::new(950.0, 750.0);
        let size: Size = Size::new(190.0, 190.0);
        let speed: f32 = 2.1;
        let strength: f32 = 11.0;
        let armor: f32 = 1.0;
        let max_health: f32 = 80.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let move_animation: Animation = create_orc_move_animation();
        let attack_animation: Animation = create_orc_attack_animation();
        let body_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: -125.0,
            w: size.w - 75.0,
            h: 145.0,
        };
        let foot_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: 50.0,
            w: size.w - 30.0,
            h: 50.0,
        };
        let enemy: Character = Character::new(
            position,
            size,
            speed,
            strength,
            armor,
            max_health,
            character_type,
            move_animation,
            attack_animation,
            body_collision,
            foot_collision,
        );
        characters.push(enemy);

        characters
    }

    fn create_barbarian_move_animation() -> Animation {
        let sprite: String = "barbarian".to_string();
        let action_type: String = "move".to_string();
        let move_frames: u8 = 8;
        let delay: u8 = 4;

        Animation::new(sprite, action_type, move_frames, delay)
    }

    fn create_barbarian_attack_animation() -> Animation {
        let sprite: String = "barbarian".to_string();
        let action_type: String = "attack".to_string();
        let move_frames: u8 = 5;
        let delay: u8 = 5;

        Animation::new(sprite, action_type, move_frames, delay)
    }

    fn create_orc_move_animation() -> Animation {
        let sprite: String = "orc".to_string();
        let action_type: String = "move".to_string();
        let move_frames: u8 = 6;
        let delay: u8 = 6;

        Animation::new(sprite, action_type, move_frames, delay)
    }

    fn create_orc_attack_animation() -> Animation {
        let sprite: String = "orc".to_string();
        let action_type: String = "attack".to_string();
        let move_frames: u8 = 5;
        let delay: u8 = 5;

        Animation::new(sprite, action_type, move_frames, delay)
    }

    pub fn spawn_first_tower() -> Character {
        let position: Position = Position::new(150.0, 250.0);

        spawn_enemy(position)
    }

    pub fn spawn_second_tower() -> Character {
        let position: Position = Position::new(1250.0, 250.0);

        spawn_enemy(position)
    }

    pub fn spawn_third_tower() -> Character {
        let position: Position = Position::new(1650.0, 250.0);

        spawn_enemy(position)
    }

    fn spawn_enemy(position: Position) -> Character {
        let size: Size = Size::new(190.0, 190.0);
        let speed: f32 = 3.1;
        let strength: f32 = 11.0;
        let armor: f32 = 1.0;
        let max_health: f32 = 80.0;
        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let move_animation: Animation = create_orc_move_animation();
        let attack_animation: Animation = create_orc_attack_animation();
        let body_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: -125.0,
            w: size.w - 75.0,
            h: 145.0,
        };
        let foot_collision: BoxCollision = BoxCollision {
            x: 0.0,
            y: 50.0,
            w: size.w - 30.0,
            h: 50.0,
        };

        Character::new(
            position,
            size,
            speed,
            strength,
            armor,
            max_health,
            character_type,
            move_animation,
            attack_animation,
            body_collision,
            foot_collision,
        )
    }
}
