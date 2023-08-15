use super::direction::Direction;
use uuid::Uuid;

#[derive(Clone)]
pub enum Action {
    MoveEntity { id: Uuid, direction: Direction },
    Attack { id: Uuid },
}
