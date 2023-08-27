use super::{
    game_draw_level_up_character::draw_level_up_character, game_draw_level_up_fire::draw_fire,
    game_draw_level_up_info::draw_character_info,
    game_draw_level_up_main_stats::draw_character_main_stats,
    game_draw_level_up_panel::draw_background_panel,
    game_draw_level_up_secondary_stats::draw_character_secondary_stats,
};
use crate::{player::player::Player, world::world_view::world_view::draw, MainState};
use ggez::{graphics::Canvas, Context};

pub fn execute(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );

    draw_level_up_character(canvas, &main_state.sprite_repository);
    draw_fire(
        canvas,
        &main_state.sprite_repository,
        main_state.level_up.turn,
    );
    let mut player: Player = main_state.player.clone();
    draw_background_panel(ctx, canvas);
    draw_character_info(ctx, canvas, &mut player);
    draw_character_main_stats(
        ctx,
        canvas,
        &mut player,
        main_state.level_up.get_selected_stat(),
    );
    draw_character_secondary_stats(ctx, canvas, &mut player);
    draw_continue_button(ctx);
}

fn draw_continue_button(ctx: &mut Context) {}
