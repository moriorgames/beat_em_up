use super::{action::Action, combat::Combat};
use crate::{
    character::{
        box_collision::BoxCollision, character::Character, character_state::CharacterState,
    },
    world::world::World,
};

impl Combat {
    pub fn process_counter_attack(
        &mut self,
        action: Action,
        characters: &mut Vec<Character>,
        world: &World,
    ) {
        if let Action::CounterAttack {
            id,
            direction,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                let bodies: Vec<Character> = characters.clone();
                for character in characters
                    .iter_mut()
                    .filter(|c: &&mut Character| c.id == id && !c.action_processed)
                {
                    if self.is_able_to_counter_attack_on_current_turn(self.turn, from, character) {
                        let world_space: BoxCollision =
                            character.next_foot_collision_to_world_space(direction);
                        if world_space.is_inside(&world.bounds) {
                            character.jump_by_direction(direction);
                        }

                        for body in bodies.iter() {
                            if let Some(weapon_collision) = character.weapon_collision.clone() {
                                if character.id != body.id {
                                    if BoxCollision::collides_with(
                                        body.position.clone(),
                                        &body.body_collision,
                                        character.position.clone(),
                                        &weapon_collision,
                                    ) {
                                        let spread_damage: f32 = character.attack_timer_hit as f32;
                                        let mut damage: f32 = (character.counter_damage
                                            - body.defense)
                                            / spread_damage;
                                        if damage <= 1.0 {
                                            damage = 1.0;
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

                    self.counter_attack_with_buffer(self.turn, from, character);
                }
            }
        }
    }

    fn is_able_to_counter_attack_on_current_turn(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
    ) -> bool {
        turn > from && character.is_counter_attacking() && !character.action_processed
    }

    fn counter_attack_with_buffer(&mut self, turn: u128, from: u128, character: &mut Character) {
        if (turn == from || turn == from + 1)
            && character
                .character_state
                .can_transition_to(&CharacterState::CounterAttacking)
            && character.current_stamina >= 10.0
        {
            character.start_counter_attack();
        }
    }
}
