use crate::{world::world_view::world_view::draw, MainState};
use ggez::{graphics::Canvas, Context};

pub fn execute(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );
}
