#[derive(Clone, Debug, PartialEq)]
pub enum CharacterTypes {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
pub enum Facing {
    Left,
    Right,
}
