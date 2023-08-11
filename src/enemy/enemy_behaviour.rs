use super::enemy_state::EnemyState;
use crate::geometry::position::Position;
use nalgebra::Point2;

pub struct EnemyBehavior;

impl EnemyBehavior {
    const ENEMY_SPEED: f32 = 1.3;

    pub fn new() -> Self {
        EnemyBehavior {}
    }

    pub fn update(&self, enemy_state: &mut EnemyState, player_position: Point2<f32>) {
        let enemy_position: Position = enemy_state.position.clone();
        let position: Position = self.seek_player(enemy_position, player_position);
        enemy_state.update_position(position);
    }

    fn seek_player(&self, enemy_position: Position, player_position: Point2<f32>) -> Position {
        let dir_x: f32 = player_position.x - enemy_position.x;
        let dir_y: f32 = player_position.y - enemy_position.y;

        let magnitude: f32 = (dir_x * dir_x + dir_y * dir_y).sqrt();
        let normalized_dir_x: f32 = dir_x / magnitude;
        let normalized_dir_y: f32 = dir_y / magnitude;

        let x: f32 = enemy_position.x + normalized_dir_x * Self::ENEMY_SPEED;
        let y: f32 = enemy_position.y + normalized_dir_y * Self::ENEMY_SPEED;

        Position::new(x, y)
    }
}
