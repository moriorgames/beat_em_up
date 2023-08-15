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

        for character in &mut *characters {
            character.update();
        }

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
                            character.update_move_animation();
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn is_colliding(a: &Character, b: &Character) -> bool {
        if a.id == b.id {
            return false;
        }

        let ax: f32 = a.position.x - a.size.w / 2.0;
        let ay: f32 = a.position.y - a.size.h / 2.0;
        let aw: f32 = a.size.w;
        let ah: f32 = a.size.h;

        let bx: f32 = b.position.x - b.size.w / 2.0;
        let by: f32 = b.position.y - b.size.h / 2.0;
        let bw: f32 = b.size.w;
        let bh: f32 = b.size.h;

        if ax + aw < bx || bx + bw < ax {
            return false;
        }
        if ay + ah < by || by + bh < ay {
            return false;
        }

        true
    }
}
