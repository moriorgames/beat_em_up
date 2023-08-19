use super::{action::Action, combat::Combat};
use crate::character::{box_collision::BoxCollision, character::Character};

impl Combat {
    pub fn process_start_attacking(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::StartAttacking { id, from, to } = action {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if character.is_idle() {
                        character.start_attacking();
                        self.actions.push(Action::Attacking {
                            id: character.id,
                            damage: 10.0,
                            from: self.turn,
                            to: self.turn + 22,
                        });
                    }
                }
            }
        }
    }

    pub fn process_attacking(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Attacking {
            id,
            damage,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                let bodies: Vec<Character> = characters.clone();
                for weapon in characters.iter_mut().filter(|c| c.id == id) {
                    for body in bodies.iter() {
                        if weapon.id != body.id {
                            if BoxCollision::collides_with(
                                body.position.clone(),
                                &body.body_collision,
                                weapon.position.clone(),
                                &weapon.weapon_collision,
                            ) {
                                let action: Action = Action::Damage {
                                    id: body.id,
                                    damage: damage,
                                    from: self.turn + 1,
                                    to: self.turn + 2,
                                };
                                self.actions.push(action);
                            }
                        }
                    }

                    if self.turn == to {
                        weapon.back_to_idle();
                    }
                }
            }
        }
    }
}
