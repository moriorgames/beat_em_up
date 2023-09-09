use super::{action::Action, combat::Combat};
use crate::character::{
    box_collision::BoxCollision, character::Character, character_state::CharacterState,
    stats::Stats,
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
                                        let mut damage: f32 = self.calculate_damage(
                                            character,
                                            body.defense,
                                            action.clone(),
                                        );
                                        if damage <= 0.5 {
                                            damage = 0.5;
                                        }
                                        if body.is_defending() {
                                            damage = 0.1;
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

                    self.attack_with_buffer(self.turn, from, character, action.clone());
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

    fn attack_with_buffer(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
        action: Action,
    ) {
        if turn == from || turn == from + 1 {
            if let Action::FastAttack {
                id: _,
                from: _,
                to: _,
            } = action
            {
                if character
                    .character_state
                    .can_transition_to(&CharacterState::FastAttacking)
                    && character.current_stamina >= Stats::MIN_STAMINA
                {
                    character.start_fast_attacking();
                }
            } else if let Action::SlowAttack {
                id: _,
                from: _,
                to: _,
            } = action
            {
                if character
                    .character_state
                    .can_transition_to(&CharacterState::SlowAttacking)
                    && character.current_stamina >= Stats::MIN_STAMINA
                {
                    character.start_slow_attacking();
                }
            }
        }
    }

    fn calculate_damage(&mut self, character: &mut Character, defense: f32, action: Action) -> f32 {
        if let Action::FastAttack {
            id: _,
            from: _,
            to: _,
        } = action
        {
            let spread_damage: f32 = character.fast_attack_timer_hit as f32;
            let damage: f32 = (character.fast_damage - defense) / spread_damage;

            return damage;
        } else if let Action::SlowAttack {
            id: _,
            from: _,
            to: _,
        } = action
        {
            let spread_damage: f32 = character.slow_attack_timer_hit as f32;
            let damage: f32 = (character.slow_damage - defense) / spread_damage;

            return damage;
        }

        return 0.0;
    }
}
