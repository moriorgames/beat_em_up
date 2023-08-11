pub mod combat {
    use crate::combat::{action::Action, direction::Direction, event::Event};
    use std::collections::VecDeque;

    pub fn process_combat_queue(events: VecDeque<Event>) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for event in events {
            match event {
                Event::MoveLeft => {
                    actions.push(Action::MoveEntity {
                        direction: Direction::Left,
                    });
                },
                Event::MoveRight => {
                    actions.push(Action::MoveEntity {
                        direction: Direction::Right,
                    });
                },
                Event::MoveUp => {
                    actions.push(Action::MoveEntity {
                        direction: Direction::Up,
                    });
                },
                Event::MoveDown => {
                    actions.push(Action::MoveEntity {
                        direction: Direction::Down,
                    });
                },
                _ => {}
            }
        }

        actions
    }
}
