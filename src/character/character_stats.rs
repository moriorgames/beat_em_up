#[derive(Clone, Debug)]
pub struct Stats {
    pub vitality: f32,
    pub strength: f32,
    pub agility: f32,
    pub resistance: f32,
}

impl Stats {
    const BASE_HEALTH: f32 = 40.0;
    const BASE_SPEED: f32 = 1.0;
    const BASE_DAMAGE: f32 = 10.0;
    const BASE_DEFENSE: f32 = 0.0;

    pub fn new(vitality: f32, strength: f32, agility: f32, resistance: f32) -> Self {
        Stats {
            vitality,
            strength,
            agility,
            resistance,
        }
    }

    pub fn get_calculated_stats(&self) -> (f32, f32, f32, f32, f32) {
        (
            self.calculate_health(),
            self.calculate_speed(),
            self.calculate_speed_jump(),
            self.calculate_damage(),
            self.calculate_defense(),
        )
    }

    fn calculate_health(&self) -> f32 {
        Self::BASE_HEALTH + self.vitality * 5.0
    }

    fn calculate_speed(&self) -> f32 {
        Self::BASE_SPEED + (self.agility * 0.6)
    }

    fn calculate_speed_jump(&self) -> f32 {
        Self::BASE_SPEED + (self.agility * 1.2)
    }

    fn calculate_damage(&self) -> f32 {
        Self::BASE_DAMAGE + (self.strength * 1.5)
    }

    fn calculate_defense(&self) -> f32 {
        Self::BASE_DEFENSE + (self.strength * 0.15) + (self.agility * 0.15) + (self.resistance * 0.5)
    }
}
