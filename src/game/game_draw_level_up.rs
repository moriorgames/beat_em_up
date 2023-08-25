use crate::{world::world_view::world_view::draw, MainState, character::character::Character, sprite::sprite_repository::SpriteRepository};
use ggez::{graphics::{Canvas, DrawParam}, Context, mint::Point2};

pub fn execute(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );

    draw_level_up_character(ctx, canvas, &main_state.player.character, &main_state.sprite_repository,);
    draw_fire(ctx);
    draw_character_stats(ctx);
    draw_continue_button(ctx);
}

fn draw_continue_button(ctx: &mut Context) {
}

fn draw_character_stats(ctx: &mut Context) {
}

fn draw_fire(ctx: &mut Context) {
}

fn draw_level_up_character(ctx: &mut Context, canvas: &mut Canvas, character: &Character, sprite_repository: &SpriteRepository) {
    let sprite_id: String = "barbarian_level_up".to_string();
    if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
        let x: f32 = 200.0;
        let y: f32 = 500.0;
        let dst: Point2<f32> = Point2 { x, y };
        let scale: Point2<f32> = Point2 { x: 3f32, y: 3f32 }; // Ajusta el tamaño según lo necesites.

        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale);
        canvas.draw(sprite, draw_params);
    }
}
