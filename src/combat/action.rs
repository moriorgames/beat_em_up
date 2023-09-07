use super::direction::Direction;
use uuid::Uuid;

pub const DAMAGE_DELAY: u8 = 4;

#[derive(Clone, Debug)]
pub enum Action {
    FastAttack {
        id: Uuid,
        from: u128,
        to: u128,
    },
    Move {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    Jump {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    BackJump {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    CounterAttack {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    Damage {
        id: Uuid,
        damage: f32,
        from: u128,
    },
}
