#[derive(Clone, Debug)]
pub struct Stats {
    strength: f32,
    agility: f32,
    vitality: f32,
    resistance: f32,
}

impl Stats {
    const BASE_SPEED: f32 = 2.0;
    const BASE_DAMAGE: f32 = 10.0;
    const BASE_DEFENSE: f32 = 1.0;
    const BASE_HEALT: f32 = 50.0;

    pub fn new(strength: f32, agility: f32, vitality: f32, resistance: f32) -> Self {
        Stats {
            strength,
            agility,
            vitality,
            resistance,
        }
    }

    pub fn get_calculated_stats(&self) -> (f32, f32, f32, f32, f32) {
        (
            self.calculate_speed(),
            self.calculate_speed_jump(),
            self.calculate_damage(),
            self.calculate_defense(),
            self.calculate_max_health(),
        )
    }

    fn calculate_speed(&self) -> f32 {
        Self::BASE_SPEED + (self.agility * 0.1)
    }

    fn calculate_speed_jump(&self) -> f32 {
        Self::BASE_SPEED + (self.agility * 0.3)
    }

    fn calculate_damage(&self) -> f32 {
        Self::BASE_DAMAGE + (self.strength * 1.5)
    }

    fn calculate_defense(&self) -> f32 {
        Self::BASE_DEFENSE + (self.agility * 0.2) + (self.resistance * 0.5)
    }

    fn calculate_max_health(&self) -> f32 {
        Self::BASE_HEALT + self.vitality * 10.0
    }
}
