use super::{action::Action, combat::Combat};
use crate::character::{
    box_collision::BoxCollision, character::Character, character_state::CharacterState,
};

impl Combat {
    pub fn process_attacking(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::FastAttack { id, from, to } | Action::SlowAttack { id, from, to } = action {
            if self.turn >= from && self.turn <= to {
                let bodies: Vec<Character> = characters.clone();
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if self.is_able_to_attack_on_current_turn(self.turn, from, character) {
                        character.attack();
                        for body in bodies.iter() {
                            if let Some(weapon_collision) = character.weapon_collision.clone() {
                                if character.id != body.id {
                                    if BoxCollision::collides_with(
                                        body.position.clone(),
                                        &body.body_collision,
                                        character.position.clone(),
                                        &weapon_collision,
                                    ) {
                                        let spread_damage: f32 =
                                            character.fast_attack_timer_hit as f32;
                                        let mut damage: f32 =
                                            (character.fast_damage - body.defense) / spread_damage;
                                        println!("character.fast_damage: {:?} body.defense: {:?} spread_damage: {:?}", character.fast_damage, body.defense,  spread_damage);
                                        if damage <= 0.5 {
                                            damage = 0.5;
                                        }
                                        let action: Action = Action::Damage {
                                            id: body.id,
                                            damage,
                                            from: self.turn + 1,
                                        };
                                        self.actions.push(action);
                                    }
                                }
                            }
                        }
                    }

                    self.attack_with_buffer(self.turn, from, character);
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
        turn > from
            && character.is_attacking()
            && !character.action_processed
            && character.current_stamina >= 0.0
    }

    fn attack_with_buffer(&mut self, turn: u128, from: u128, character: &mut Character) {
        if (turn == from || turn == from + 1)
            && character
                .character_state
                .can_transition_to(&CharacterState::Attacking)
            && character.current_stamina >= 10.0
        {
            character.start_attacking();
        }
    }
}
