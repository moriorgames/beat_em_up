use crate::combat::action::Action;

pub struct LevelUp {
    pub turn: u128,
    pub actions: Vec<Action>,
}

impl LevelUp {
    pub fn new() -> Self {
        Self {
            turn: 0,
            actions: Vec::new(),
        }
    }

    pub fn process(&mut self) {
        self.turn += 1;
        println!("Level up Turn: {:?}", self.turn);
    }
}
