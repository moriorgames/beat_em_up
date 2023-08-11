use super::direction::Direction;
use uuid::Uuid;

pub enum Action {
    MoveEntity { id: Uuid, direction: Direction },
}
