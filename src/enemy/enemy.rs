use super::enemy_behaviour::EnemyBehavior;
use super::enemy_state::EnemyState;
use super::enemy_transformation::EnemyTransformation;
use super::enemy_view::EnemyView;
use ggez::graphics::Canvas;
use ggez::Context;
use nalgebra::Point2;

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

    pub fn update(&mut self, player_position: Point2<f32>) {
        self.behaviour.update(&mut self.state, player_position);
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let enemy_transformation: EnemyTransformation =
            EnemyTransformation::from_enemy(&self.state);
        let _ = self.view.draw(ctx, canvas, enemy_transformation);
    }
}
