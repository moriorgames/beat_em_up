#[derive(Clone, Debug, PartialEq)]
pub enum CharacterState {
    Idle,
    Moving,
    Jumping,
    BackJumping,
    FastAttacking,
    SlowAttacking,
    CounterAttacking,
    Defending,
}

impl CharacterState {
    pub fn can_transition_to(&self, next_state: &CharacterState) -> bool {
        match self {
            CharacterState::Idle => true,
            CharacterState::Moving => true,
            CharacterState::FastAttacking => matches!(next_state, CharacterState::Idle),
            CharacterState::SlowAttacking => matches!(next_state, CharacterState::Idle),
            CharacterState::CounterAttacking => matches!(next_state, CharacterState::Idle),
            CharacterState::Jumping => matches!(next_state, CharacterState::Idle),
            CharacterState::BackJumping => matches!(next_state, CharacterState::Idle),
            CharacterState::Defending => true,
        }
    }
}
