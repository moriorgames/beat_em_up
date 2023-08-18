use super::direction::Direction;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Action {
    StartMoving {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    Moving {
        id: Uuid,
        direction: Direction,
        from: u128,
        to: u128,
    },
    Attack {
        id: Uuid,
        from: u128,
        to: u128,
    },
    Damage {
        id: Uuid,
        damage: f32,
        from: u128,
        to: u128,
    },
}
