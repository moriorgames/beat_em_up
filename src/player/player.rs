use super::player_controls::PlayerControls;
use super::player_state::PlayerState;
use super::player_view::PlayerView;
use crate::combat::action::Action;
use crate::combat::direction::Direction;
use crate::combat::event::Event;
use crate::geometry::position::Position;
use ggez::graphics::Canvas;
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

    pub fn update(&mut self, ctx: &mut Context) -> Vec<Event> {
        self.controls.handle_input(ctx, self.state.id)
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let position: Position = self.state.position.clone();
        let size: crate::geometry::size::Size = self.state.size.clone();
        let _ = self.view.draw(ctx, canvas, position, size);
    }

    pub fn apply_game_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            match action {
                Action::MoveEntity { id, direction } => {
                    if id == self.state.id {
                        self.move_player_in_direction(direction);
                    }
                }
            }
        }
    }

    fn move_player_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.state.move_left(),
            Direction::Right => self.state.move_right(),
            Direction::Up => self.state.move_up(),
            Direction::Down => self.state.move_down(),
        }
    }
}
