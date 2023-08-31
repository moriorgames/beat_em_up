use super::{action::Action, combat::Combat};
use crate::{
    character::{
        box_collision::BoxCollision, character::Character, character_state::CharacterState,
    },
    world::world::World,
};

impl Combat {
    pub fn process_jumping(
        &mut self,
        action: Action,
        characters: &mut Vec<Character>,
        world: &World,
    ) {
        if let Action::Jump {
            id,
            direction,
            from,
            to,
        }
        | Action::BackJump {
            id,
            direction,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                for character in characters
                    .iter_mut()
                    .filter(|c: &&mut Character| c.id == id && !c.action_processed)
                {
                    if self.is_able_to_jump_on_current_turn(self.turn, from, character) {
                        let world_space: BoxCollision =
                            character.next_foot_collision_to_world_space(direction);
                        if world_space.is_inside(&world.bounds) {
                            character.jump_by_direction(direction);
                        }
                    }

                    self.jump_with_buffer(self.turn, from, character, action.clone());
                }
            }
        }
    }

    fn is_able_to_jump_on_current_turn(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
    ) -> bool {
        turn > from && (character.is_jumping() || character.is_back_jumping())
    }

    fn jump_with_buffer(
        &mut self,
        turn: u128,
        from: u128,
        character: &mut Character,
        action: Action,
    ) {
        if turn == from
            && character
                .character_state
                .can_transition_to(&CharacterState::Jumping)
            && !character.action_processed
            && character.current_stamina >= 10.0
        {
            match action {
                Action::Jump {
                    id: _,
                    direction: _,
                    from: _,
                    to: _,
                } => {
                    character.start_jumping();
                }
                Action::BackJump {
                    id: _,
                    direction: _,
                    from: _,
                    to: _,
                } => {
                    character.start_back_jumping();
                }
                _ => {}
            }
        }
    }
}
