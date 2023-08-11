pub mod combat {
    use crate::combat::{action::Action, direction::Direction, event::Event};
    use std::collections::VecDeque;

    pub fn process_combat_queue(events: VecDeque<Event>) -> Vec<Action> {
        events
            .iter()
            .filter_map(|event: &Event| match event {
                Event::MoveLeft { id } => Some(Action::MoveEntity {
                    id: *id,
                    direction: Direction::Left,
                }),
                Event::MoveRight { id } => Some(Action::MoveEntity {
                    id: *id,
                    direction: Direction::Right,
                }),
                Event::MoveUp { id } => Some(Action::MoveEntity {
                    id: *id,
                    direction: Direction::Up,
                }),
                Event::MoveDown { id } => Some(Action::MoveEntity {
                    id: *id,
                    direction: Direction::Down,
                }),
                _ => None,
            })
            .collect()
    }
}
