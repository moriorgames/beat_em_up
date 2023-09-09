use super::{character::Character, character_state::CharacterState, stats::Stats};

impl Character {
    pub fn start_moving(&mut self) {
        self.character_state = CharacterState::Moving
    }

    pub fn start_defending(&mut self) {
        self.character_state = CharacterState::Defending
    }

    pub fn start_jumping(&mut self) {
        self.jump_timer = self.full_jump_timer;
        self.action_processed = true;
        self.character_state = CharacterState::Jumping;
        self.reduce_stamina(Stats::STAMINA_COST);
    }

    pub fn start_back_jumping(&mut self) {
        self.jump_timer = self.full_jump_timer;
        self.action_processed = true;
        self.character_state = CharacterState::BackJumping;
        self.reduce_stamina(Stats::STAMINA_COST);
    }

    pub fn start_fast_attacking(&mut self) {
        self.attack_timer = self.fast_attack_timer;
        self.character_state = CharacterState::FastAttacking;
        self.reduce_stamina(Stats::STAMINA_COST);
    }

    pub fn start_slow_attacking(&mut self) {
        self.attack_timer = self.slow_attack_timer;
        self.character_state = CharacterState::SlowAttacking;
        self.reduce_stamina(Stats::STAMINA_COST);
    }

    pub fn start_counter_attack(&mut self) {
        self.attack_timer = self.fast_attack_timer;
        self.character_state = CharacterState::CounterAttacking;
        self.reduce_stamina(Stats::STAMINA_COST);
    }
}
