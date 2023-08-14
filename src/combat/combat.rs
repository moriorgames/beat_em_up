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
                    match action {
                        Action::MoveEntity { id, direction: _ } => {
                            if let Some(main_character) =
                                characters.iter().find(|&char| char.id == id)
                            {
                                let has_collision: bool =
                                    characters.iter().any(|character: &Character| {
                                        is_colliding(&main_character, character)
                                    });

                                if !has_collision {
                                    actions.push(action);
                                }
                            }
                        }
                        _ => actions.push(action),
                    }
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

    fn is_colliding(a: &Character, b: &Character) -> bool {
        if a.id == b.id {
            return false;
        }

        let a_left: f32 = a.position.x;
        let a_right: f32 = a.position.x + a.size.w;
        let a_top: f32 = a.position.y;
        let a_bottom: f32 = a.position.y + a.size.h;

        let b_left: f32 = b.position.x;
        let b_right: f32 = b.position.x + b.size.w;
        let b_top: f32 = b.position.y;
        let b_bottom: f32 = b.position.y + b.size.h;

        !(a_left > b_right || a_right < b_left || a_top > b_bottom || a_bottom < b_top)
    }
}
