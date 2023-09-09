use super::{action::Action, combat::Combat};
use crate::character::{
    character::Character, character_state::CharacterState,
};

impl Combat {
    pub fn process_defending(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Defense { id, from, to } = action {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if self.is_able_to_defend_on_current_turn(self.turn, from, character) {}

                    if self.turn == from && character.is_idle() {
                        println!(" --------------- Defending!!!!!");
                        character.start_defending();
                    }

                    if self.turn == to && character.is_defending() {
                        character.back_to_idle();
                    }
                }
            }
        }
    }

    fn is_able_to_defend_on_current_turn(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
    ) -> bool {
        turn > from
            && character
                .character_state
                .can_transition_to(&CharacterState::Defending)
            && !character.action_processed
    }
}
