use super::enemy_state::EnemyState;
use nalgebra::Point2;

#[derive(Clone)]
pub struct EnemyTransformation {
    pub position: Point2<f32>,
    pub health_percentage: f32,
}

impl EnemyTransformation {
    pub fn from_enemy(state: &EnemyState) -> Self {
        let health_percentage: f32 = state.current_health as f32 / state.max_health as f32;

        EnemyTransformation {
            position: state.position.clone(),
            health_percentage,
        }
    }
}
