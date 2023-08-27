use crate::player::level_up_controls::{LevelUpControls, LevelUpIntention};
use ggez::Context;

pub struct LevelUp {
    pub turn: u128,
    pub level_up_controls: LevelUpControls,
}

impl LevelUp {
    pub fn new() -> Self {
        let level_up_controls: LevelUpControls = LevelUpControls::new();
        Self {
            turn: 0,
            level_up_controls,
        }
    }

    pub fn process(&mut self, ctx: &mut Context) {
        self.turn += 1;
        println!("Level up Turn: {:?}", self.turn);

        let intention: LevelUpIntention = self.level_up_controls.handle_input(ctx);
        println!("{:?}", intention);
    }
}
