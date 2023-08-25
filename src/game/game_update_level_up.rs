use ggez::Context;
use crate::{MainState, TARGET_FPS};

pub fn execute(ctx: &mut Context, main_state: &mut MainState) {
    while ctx.time.check_update_time(TARGET_FPS) {
        main_state.level_up.process();
    }
}
