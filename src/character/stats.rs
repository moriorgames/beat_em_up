#[derive(Clone, Debug)]
pub struct Stats {
    pub vitality: f32,
    pub strength: f32,
    pub agility: f32,
    pub resistance: f32,
}

impl Stats {
    const BASE_HEALTH: f32 = 100.0;
    const BASE_JUMP: f32 = 3.0;
    const BASE_DAMAGE: f32 = 30.0;
    const COUNTER_DAMAGE: f32 = 19.0;
    const BASE_DEFENSE: f32 = 0.5;
    const BASE_STAMINA: f32 = 40.0;
    pub const STAMINA_COST: f32 = 20.0;
    pub const MIN_STAMINA: f32 = 10.0;

    pub fn new(vitality: f32, strength: f32, agility: f32, resistance: f32) -> Self {
        Stats {
            vitality,
            strength,
            agility,
            resistance,
        }
    }

    pub fn get_calculated_stats(&self) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        (
            self.calculate_health(),
            self.calculate_speed(),
            self.calculate_speed_jump(),
            self.calculate_fast_damage(),
            self.calculate_slow_damage(),
            self.calculate_counter_damage(),
            self.calculate_defense(),
            self.calculate_stamina(),
        )
    }

    fn calculate_health(&self) -> f32 {
        Self::BASE_HEALTH + self.vitality * 15.0
    }

    fn calculate_speed(&self) -> f32 {
        let speed: f32 = match self.agility as u32 {
            1..=9 => 2.0,
            10..=25 => 2.5,
            26..=45 => 2.9,
            46..=75 => 3.1,
            76..=99 => 3.7,
            _ => 1.7,
        };

        speed
    }

    fn calculate_speed_jump(&self) -> f32 {
        Self::BASE_JUMP + self.calculate_speed()
    }

    fn calculate_fast_damage(&self) -> f32 {
        Self::BASE_DAMAGE + (self.agility * 3.0)
    }

    fn calculate_slow_damage(&self) -> f32 {
        Self::BASE_DAMAGE + (self.strength * 5.0)
    }

    fn calculate_counter_damage(&self) -> f32 {
        Self::COUNTER_DAMAGE + self.agility + self.strength
    }

    fn calculate_defense(&self) -> f32 {
        Self::BASE_DEFENSE + (self.strength * 0.5) + (self.agility * 0.4) + self.resistance
    }

    fn calculate_stamina(&self) -> f32 {
        Self::BASE_STAMINA + (self.resistance * 2.5)
    }
}
