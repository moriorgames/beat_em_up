pub mod enemy_behavior {
    use crate::{
        character::{character::Character, character_types::CharacterTypes},
        combat::{action::Action, direction::Direction},
        geometry::position::Position,
    };
    use uuid::Uuid;

    pub fn update_enemy_behaviour(
        characters: Vec<Character>,
        player_position: Position,
        turn: u128,
    ) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for character in &characters {
            if let CharacterTypes::Enemy = character.character_type {
                seek_player(&mut actions, character, player_position.clone(), turn);
            }
        }

        actions
    }

    fn seek_player(
        actions: &mut Vec<Action>,
        character: &Character,
        player_position: Position,
        turn: u128,
    ) {
        let enemy_position: Position = character.position.clone();
        let dir_x: f32 = player_position.x - enemy_position.x;
        let dir_y: f32 = player_position.y - enemy_position.y;

        let distance: f32 = (dir_x.powi(2) + dir_y.powi(2)).sqrt();
        if distance <= 110.0 {
            actions.push(Action::Attacking {
                id: character.id,
                from: turn + 1,
                to: turn + 22,
            });
        } else {
            let magnitude: f32 = (dir_x * dir_x + dir_y * dir_y).sqrt();
            let normalized_dir_x: f32 = dir_x / magnitude;
            let normalized_dir_y: f32 = dir_y / magnitude;

            let id: Uuid = character.id;
            let direction = match (normalized_dir_x, normalized_dir_y) {
                (x, y) if x > 0.0 && y > 0.0 => Direction::DownRight,
                (x, y) if x > 0.0 && y < 0.0 => Direction::UpRight,
                (x, y) if x < 0.0 && y > 0.0 => Direction::DownLeft,
                (x, y) if x < 0.0 && y < 0.0 => Direction::UpLeft,
                (x, _) if x > 0.0 => Direction::Right,
                (x, _) if x < 0.0 => Direction::Left,
                (_, y) if y > 0.0 => Direction::Down,
                (_, y) if y < 0.0 => Direction::Up,
                _ => return,
            };

            actions.push(Action::Moving {
                id,
                direction,
                from: turn + 1,
                to: turn + 20,
            });
        }
    }
}
