pub mod enemy_behavior {
    use crate::{
        character::{character::Character, character_types::CharacterTypes},
        combat::event::Event,
        geometry::position::Position,
    };
    use uuid::Uuid;

    pub fn update_enemy_behaviour(
        characters: Vec<Character>,
        player_position: Position,
    ) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        for character in &characters {
            if let CharacterTypes::Enemy = character.character_type {
                seek_player(&mut events, character, player_position.clone());
            }
        }

        events
    }

    fn seek_player(events: &mut Vec<Event>, character: &Character, player_position: Position) {
        let enemy_position: Position = character.position.clone();
        let dir_x: f32 = player_position.x - enemy_position.x;
        let dir_y: f32 = player_position.y - enemy_position.y;

        let magnitude: f32 = (dir_x * dir_x + dir_y * dir_y).sqrt();
        let normalized_dir_x: f32 = dir_x / magnitude;
        let normalized_dir_y: f32 = dir_y / magnitude;

        let id: Uuid = character.id;
        if normalized_dir_x > 0.0 {
            events.push(Event::MoveRight { id });
        } else if normalized_dir_x < 0.0 {
            events.push(Event::MoveLeft { id });
        }

        if normalized_dir_y > 0.0 {
            events.push(Event::MoveDown { id });
        } else if normalized_dir_y < 0.0 {
            events.push(Event::MoveUp { id });
        }
    }
}
