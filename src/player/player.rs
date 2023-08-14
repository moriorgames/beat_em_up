use super::player_state::PlayerState;
use super::player_view::PlayerView;
use crate::behaviours::movable::Movable;
use crate::combat::action::Action;
use crate::combat::direction::Direction;
use crate::geometry::position::Position;
use ggez::graphics::Canvas;
use ggez::Context;

pub struct Player {
    pub state: PlayerState,
    view: PlayerView,
}

impl Player {
    pub fn new() -> Self {
        Player {
            state: PlayerState::new(),
            view: PlayerView::new(),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let position: Position = self.state.position.clone();
        let size: crate::geometry::size::Size = self.state.size.clone();
        let _ = self.view.draw(ctx, canvas, position, size);
    }

    pub fn get_last_player_position(&mut self) -> Position {
        self.state.position.clone()
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
