use crate::MainState;
use ggez::Context;

pub fn execute(ctx: &mut Context, main_state: &mut MainState) {
    main_state.level_up.process(ctx, &mut main_state.player);
}
