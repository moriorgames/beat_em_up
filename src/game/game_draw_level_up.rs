use crate::{MainState, world::world_view::world_view::draw};
use ggez::{graphics::Canvas, Context};

pub fn execute(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    let _ = draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );
}
