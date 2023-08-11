use super::enemy_state::EnemyState;
use crate::geometry::position::Position;
use crate::geometry::size::Size;

#[derive(Clone)]
pub struct EnemyTransformation {
    pub position: Position,
    pub size: Size,
    pub health_percentage: f32,
}

impl EnemyTransformation {
    pub fn from_enemy(state: &EnemyState) -> Self {
        let health_percentage: f32 = state.current_health as f32 / state.max_health as f32;

        EnemyTransformation {
            position: state.position.clone(),
            size: state.size.clone(),
            health_percentage,
        }
    }
}
