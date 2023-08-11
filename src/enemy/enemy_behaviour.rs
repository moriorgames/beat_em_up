use super::enemy_state::EnemyState;
use crate::{combat::event::Event, geometry::position::Position};
use uuid::Uuid;

pub struct EnemyBehavior;

impl EnemyBehavior {
    pub fn new() -> Self {
        EnemyBehavior {}
    }

    pub fn update(&self, state: &mut EnemyState, player_position: Position) -> Vec<Event> {
        self.seek_player(state, player_position)
    }

    fn seek_player(&self, state: &mut EnemyState, player_position: Position) -> Vec<Event> {
        let mut events: Vec<Event> = Vec::new();

        let enemy_position: Position = state.position.clone();
        let dir_x: f32 = player_position.x - enemy_position.x;
        let dir_y: f32 = player_position.y - enemy_position.y;

        let magnitude: f32 = (dir_x * dir_x + dir_y * dir_y).sqrt();
        let normalized_dir_x: f32 = dir_x / magnitude;
        let normalized_dir_y: f32 = dir_y / magnitude;

        let id: Uuid = state.id;
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

        events
    }
}
