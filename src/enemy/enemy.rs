use super::enemy_behaviour::EnemyBehavior;
use super::enemy_state::EnemyState;
use super::enemy_transformation::EnemyTransformation;
use super::enemy_view::EnemyView;
use crate::combat::action::Action;
use crate::combat::direction::Direction;
use crate::combat::event::Event;
use crate::geometry::position::Position;
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

    pub fn update(&mut self, player_position: Position) -> Vec<Event> {
        self.behaviour.update(&mut self.state, player_position)
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) {
        let enemy_transformation: EnemyTransformation =
            EnemyTransformation::from_enemy(&self.state);
        let _ = self.view.draw(ctx, canvas, enemy_transformation);
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
