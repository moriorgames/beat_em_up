use super::enemy_behaviour::EnemyBehavior;
use super::enemy_state::EnemyState;
use super::enemy_view::EnemyView;
use ggez::graphics::Canvas;
use ggez::Context;

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

    pub fn update(&mut self, ctx: &mut Context) {}

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let _ = self.view.draw(ctx, canvas, self.state.position.clone());
    }
}
