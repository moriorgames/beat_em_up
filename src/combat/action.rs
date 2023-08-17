use super::direction::Direction;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Action {
    MoveEntity { id: Uuid, direction: Direction },
    Attack { id: Uuid },
    Damage { id: Uuid, damage: f32 },
}
