use super::{character::Character, character_state::CharacterState};

impl Character {
    pub fn is_idle(&self) -> bool {
        self.character_state == CharacterState::Idle
    }

    pub fn is_moving(&self) -> bool {
        self.character_state == CharacterState::Moving
    }

    pub fn is_defending(&self) -> bool {
        self.character_state == CharacterState::Defending
    }

    pub fn is_jumping(&self) -> bool {
        self.character_state == CharacterState::Jumping
    }

    pub fn is_back_jumping(&self) -> bool {
        self.character_state == CharacterState::BackJumping
    }

    pub fn is_attacking(&self) -> bool {
        self.character_state == CharacterState::FastAttacking
            || self.character_state == CharacterState::SlowAttacking
    }

    pub fn is_counter_attacking(&self) -> bool {
        self.character_state == CharacterState::CounterAttacking
    }
}
