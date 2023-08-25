use crate::character::character::Character;

pub struct Player {
    pub character: Character,
    experience: u32,
    level: u32,
}

impl Player {
    pub fn new(character: Character) -> Self {
        Self {
            character,
            experience: 0,
            level: 1,
        }
    }
}
