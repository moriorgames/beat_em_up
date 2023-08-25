use crate::{
    sprite::sprite_repository::SpriteRepository, world::world_view::world_view::draw, MainState, player::player::Player, character::character_stats::Stats,
};
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text},
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

    draw_level_up_character(canvas, &main_state.sprite_repository);
    draw_fire(
        canvas,
        &main_state.sprite_repository,
        main_state.level_up.turn,
    );
    let mut player: Player = main_state.player.clone();
    draw_character_stats(ctx, canvas, &mut player);
    draw_continue_button(ctx);
}

fn draw_level_up_character(canvas: &mut Canvas, sprite_repository: &SpriteRepository) {
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

fn draw_fire(canvas: &mut Canvas, sprite_repository: &SpriteRepository, frame_count: u128) {
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

fn draw_character_stats(ctx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
    let mode: DrawMode = DrawMode::fill();
    let bounds: Rect = Rect::new(800.0, 100.0, 900.0, 800.0);
    let color: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    let draw_params: DrawParam = DrawParam::new();
    let panel: Mesh = Mesh::new_rectangle(ctx, mode, bounds, color).unwrap();

    canvas.draw(&panel, draw_params);

    let left_panel_x: f32 = 820.0;
    let line_height: f32 = 30.0;

    let stats_before: Stats = player.character.stats.clone();
    let stats_after: Stats = player.character.stats.clone();

    let stat_labels: Vec<&str> = vec!["Fuerza", "Agilidad", "Vitalidad", "Resistencia"];
    let stat_values_before: Vec<f32> = vec![stats_before.strength, stats_before.agility, stats_before.vitality, stats_before.resistance];
    let stat_values_after: Vec<f32> = vec![stats_after.strength, stats_after.agility, stats_after.vitality, stats_after.resistance];

    for (i, label) in stat_labels.iter().enumerate() {
        let y: f32 = 120.0 + i as f32 * line_height;
        let text_before: Text = Text::new(format!("{}: {:.0} => {:.0}", label, stat_values_before[i], stat_values_after[i]));
        let params: DrawParam = DrawParam::new().dest(Point2 { x: left_panel_x, y });
        canvas.draw(&text_before, params);
    }
}

fn draw_continue_button(ctx: &mut Context) {}
