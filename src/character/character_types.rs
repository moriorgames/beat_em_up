#[derive(Clone, PartialEq)]
pub enum CharacterTypes {
    Player,
    Enemy,
}

#[derive(Clone)]
pub enum Facing {
    Left,
    Right,
}
