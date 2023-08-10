use super::player_state::PlayerState;
use super::player_controls::PlayerControls;
use super::player_view::PlayerView;
use ggez::Context;

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

    pub fn update(&mut self, _ctx: &Context) {
        self.controls.handle_input();
        self.state.update();
    }

    pub fn draw(&self, _ctx: &Context) {
        self.view.draw();
    }
}
