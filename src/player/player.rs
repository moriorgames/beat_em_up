use crate::character::character::Character;

#[derive(Clone, Debug)]
pub struct Player {
    pub character: Character,
    pub level: u32,
    pub experience: u32,
    pub max_experience: u32,
}

impl Player {
    pub fn new(character: Character) -> Self {
        Self {
            character,
            level: 1,
            experience: 2,
            max_experience: 2,
        }
    }
}
