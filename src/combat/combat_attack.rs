use super::{action::Action, combat::Combat};
use crate::character::{box_collision::BoxCollision, character::Character};

impl Combat {
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
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if self.is_able_to_attack_on_current_turn(self.turn, from, character) {
                        character.attack();
                        for body in bodies.iter() {
                            if character.id != body.id {
                                if BoxCollision::collides_with(
                                    body.position.clone(),
                                    &body.body_collision,
                                    character.position.clone(),
                                    &character.weapon_collision,
                                ) {
                                    let action: Action = Action::Damage {
                                        id: body.id,
                                        damage,
                                        from: self.turn + 1,
                                        to: self.turn + 2,
                                    };
                                    self.actions.push(action);
                                }
                            }
                        }
                    }

                    if self.turn == from && character.is_idle() {
                        character.start_attacking();
                    }

                    if self.turn == to && character.is_attacking() {
                        character.back_to_idle();
                    }
                }
            }
        }
    }

    fn is_able_to_attack_on_current_turn(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
    ) -> bool {
        turn > from && character.is_attacking() && !character.has_processed_action
    }
}
