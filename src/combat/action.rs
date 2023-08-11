use super::direction::Direction;

pub enum Action {
    MoveEntity { direction: Direction },
}
