pub mod combat {
    use crate::{
        character::character::Character,
        combat::{
            action::Action,
            event::Event,
            move_handlers::move_handlers::{down, left, right, up},
        },
    };
    use std::collections::VecDeque;

    const EVENT_HANDLERS: [fn(&Event) -> Option<Action>; 4] = [left, right, up, down];

    pub fn process_combat_queue(events: VecDeque<Event>, characters: &mut Vec<Character>) {
        let mut actions: Vec<Action> = Vec::new();

        for event in &events {
            for handler in &EVENT_HANDLERS {
                if let Some(action) = handler(event) {
                    actions.push(action);
                }
            }
        }

        for action in actions {
            for character in &mut *characters {
                match action {
                    Action::MoveEntity { id, direction } => {
                        if id == character.id {
                            character.move_by_direction(direction);
                        }
                    }
                }
            }
        }
    }
}
