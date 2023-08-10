use super::enemy_behaviour::EnemyBehavior;
use super::enemy_state::EnemyState;
use super::enemy_view::EnemyView;

pub struct Enemy {
    state: EnemyState,
    behaviour: EnemyBehavior,
    view: EnemyView,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            state: EnemyState::new(),
            behaviour: EnemyBehavior::new(),
            view: EnemyView::new(),
        }
    }
}
