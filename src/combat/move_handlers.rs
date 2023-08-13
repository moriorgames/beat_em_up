pub mod move_handlers {
    use crate::combat::{action::Action, direction::Direction, event::Event};

    pub fn left(event: &Event) -> Option<Action> {
        if let Event::MoveLeft { id } = event {
            return Some(Action::MoveEntity {
                id: *id,
                direction: Direction::Left,
            });
        }
        None
    }

    pub fn right(event: &Event) -> Option<Action> {
        if let Event::MoveRight { id } = event {
            return Some(Action::MoveEntity {
                id: *id,
                direction: Direction::Right,
            });
        }
        None
    }

    pub fn up(event: &Event) -> Option<Action> {
        if let Event::MoveUp { id } = event {
            return Some(Action::MoveEntity {
                id: *id,
                direction: Direction::Up,
            });
        }
        None
    }

    pub fn down(event: &Event) -> Option<Action> {
        if let Event::MoveDown { id } = event {
            return Some(Action::MoveEntity {
                id: *id,
                direction: Direction::Down,
            });
        }
        None
    }
}
