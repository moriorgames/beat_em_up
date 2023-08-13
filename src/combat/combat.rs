pub mod combat {
    use crate::combat::{
        action::Action,
        event::Event,
        move_handlers::move_handlers::{down, left, right, up},
    };
    use std::collections::VecDeque;

    const EVENT_HANDLERS: [fn(&Event) -> Option<Action>; 4] = [left, right, up, down];

    pub fn process_combat_queue(events: VecDeque<Event>) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for event in &events {
            for handler in &EVENT_HANDLERS {
                if let Some(action) = handler(event) {
                    actions.push(action);
                }
            }
        }

        actions
    }
}
