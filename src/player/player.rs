use super::player_controls::PlayerControls;
use super::player_state::PlayerState;
use super::player_view::PlayerView;
use ggez::graphics::Canvas;
use ggez::Context;
use nalgebra::Point2;

pub struct Player {
    state: PlayerState,
    controls: PlayerControls,
    view: PlayerView,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::new(),
            controls: PlayerControls::new(),
            view: PlayerView::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> Point2<f32> {
        self.controls.handle_input(ctx, &mut self.state);

        self.state.position
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let _ = self.view.draw(ctx, canvas, self.state.position.clone());
    }
}