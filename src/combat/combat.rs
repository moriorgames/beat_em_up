pub mod combat {
    use crate::{
        character::{
            box_collision::BoxCollision,
            character::{Character, CharacterState},
        },
        combat::{
            action::Action,
            attack_handlers::attack_handlers::attack,
            event::Event,
            move_handlers::move_handlers::{down, left, right, up},
        },
        world::world::World,
    };
    use std::collections::VecDeque;

    const EVENT_HANDLERS: [fn(&Event) -> Option<Action>; 5] = [left, right, up, down, attack];

    pub fn process_combat_queue(
        mut events: VecDeque<Event>,
        characters: &mut Vec<Character>,
        world: &World,
    ) {
        let mut actions: Vec<Action> = Vec::new();

        for character in &mut *characters {
            if let Some(event) = character.update() {
                events.push_back(event);
            }
        }

        for event in &events {
            for handler in &EVENT_HANDLERS {
                if let Some(action) = handler(event) {
                    match action {
                        Action::MoveEntity { id, direction } => {
                            if let Some(character) = characters.iter().find(|&char| char.id == id) {
                                let world_space: BoxCollision =
                                    character.next_foot_collision_to_world_space(direction);
                                if world_space.is_inside(&world.bounds) {
                                    actions.push(action);
                                }
                            }
                        }
                        Action::Attack { id } => {
                            if let Some(character) = characters.iter().find(|&char| char.id == id) {
                                if character.character_state == CharacterState::Moving {
                                    actions.push(action);
                                }
                            }

                            for body in characters.clone() {
                                for weapon in characters.clone() {
                                    if body.id != weapon.id {
                                        if BoxCollision::collides_with(
                                            body.position.clone(),
                                            &body.body_collision,
                                            weapon.position.clone(),
                                            &weapon.weapon_collision,
                                        ) {
                                            let action: Action = Action::Damage {
                                                id: body.id,
                                                damage: 20.0,
                                            };
                                            actions.push(action);
                                        }
                                    }
                                }
                            }
                        }
                        _ => (),
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
                    Action::Attack { id } => {
                        if id == character.id {
                            character.attack();
                        }
                    }
                    Action::Damage { id, damage } => {
                        if id == character.id {
                            character.apply_damage(damage)
                        }
                    }
                }
            }
        }
    }
}
