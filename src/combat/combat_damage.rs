use super::{action::Action, combat::Combat};
use crate::character::character::Character;

impl Combat {
    pub fn process_damage(&mut self, action: Action, characters: &mut Vec<Character>) {
        if let Action::Damage { id, damage, from } = action {
            if self.turn == from {
                for character in characters.iter_mut().filter(|c| c.id == id) {
                    println!("Damage: {:?}", action.clone());
                    character.start_damaged(damage)
                }
            }
        }
    }
}
