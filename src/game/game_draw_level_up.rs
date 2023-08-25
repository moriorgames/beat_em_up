use crate::{
    sprite::sprite_repository::SpriteRepository, world::world_view::world_view::draw, MainState,
};
use ggez::{
    graphics::{Canvas, Color, DrawParam, Image},
    mint::Point2,
    Context,
};

pub fn execute(ctx: &mut Context, canvas: &mut Canvas, main_state: &mut MainState) {
    draw(
        ctx,
        canvas,
        &main_state.world,
        &main_state.sprite_repository,
    );

    draw_level_up_character(ctx, canvas, &main_state.sprite_repository);
    draw_fire(
        ctx,
        canvas,
        &main_state.sprite_repository,
        main_state.level_up.turn,
    );
    draw_character_stats(ctx);
    draw_continue_button(ctx);
}

fn draw_level_up_character(
    ctx: &mut Context,
    canvas: &mut Canvas,
    sprite_repository: &SpriteRepository,
) {
    let sprite_id: String = "barbarian_level_up".to_string();
    if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
        let x: f32 = 150.0;
        let y: f32 = 250.0;
        let dst: Point2<f32> = Point2 { x, y };
        let scale: Point2<f32> = Point2 { x: 3f32, y: 3f32 };

        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale);
        canvas.draw(sprite, draw_params);
    }
}

fn draw_fire(
    ctx: &mut Context,
    canvas: &mut Canvas,
    sprite_repository: &SpriteRepository,
    frame_count: u128,
) {
    let frame_index: u128 = (frame_count / 3) % 8;
    let sprite_id: String = format!("fire_{}", frame_index);
    if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
        let x: f32 = 340.0;
        let y: f32 = 300.0;
        let dst: Point2<f32> = Point2 { x, y };
        let scale: Point2<f32> = Point2 { x: 2f32, y: 2f32 };

        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale);
        canvas.draw(sprite, draw_params);

        let dst: Point2<f32> = Point2 { x: 0.0, y: 0.0 };
        let color: Color = Color::new(1.0, 1.0, 1.0, 0.80);
        let mask_image: &Image = sprite_repository.get_sprite("fire_mask").unwrap();
        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale).color(color);
        canvas.draw(mask_image, draw_params);
    }
}

fn draw_continue_button(ctx: &mut Context) {}

fn draw_character_stats(ctx: &mut Context) {}
