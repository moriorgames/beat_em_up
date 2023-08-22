use super::{action::Action, combat::Combat};
use crate::{
    character::{box_collision::BoxCollision, character::Character},
    world::world::World,
};

impl Combat {
    pub fn process_jumping(
        &mut self,
        action: Action,
        characters: &mut Vec<Character>,
        world: &World,
    ) {
        if let Action::Jumping {
            id,
            direction,
            from,
            to,
        } = action
        {
            if self.turn >= from && self.turn <= to {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    if self.is_able_to_jump_on_current_turn(self.turn, from, character) {
                        let world_space: BoxCollision =
                            character.next_foot_collision_to_world_space(direction);
                        if world_space.is_inside(&world.bounds) {
                            character.jump_by_direction(direction);
                        }
                    }

                    if self.turn == from && character.is_idle() {
                        character.start_jumping();
                    }

                    if self.turn == to && character.is_jumping() {
                        character.back_to_idle();
                    }
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
        turn > from && character.is_jumping() && !character.has_processed_action
    }
}
