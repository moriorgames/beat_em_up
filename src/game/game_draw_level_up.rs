use super::{
    game_draw_level_up_character::draw_level_up_character, game_draw_level_up_fire::draw_fire,
};
use crate::{
    character::character_stats::Stats, player::player::Player, world::world_view::world_view::draw,
    MainState,
};
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
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
    draw_background_panel(ctx, canvas);
    draw_character_stats(ctx, canvas, &mut player);
    draw_continue_button(ctx);
}

fn draw_background_panel(ctx: &mut Context, canvas: &mut Canvas) {
    let mode: DrawMode = DrawMode::fill();
    let bounds: Rect = Rect::new(800.0, 100.0, 900.0, 800.0);
    let color: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    let draw_params: DrawParam = DrawParam::new();
    let panel: Mesh = Mesh::new_rectangle(ctx, mode, bounds, color).unwrap();

    canvas.draw(&panel, draw_params);
}

fn draw_character_stats(ctx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
    let left_panel_x: f32 = 820.0;
    let line_height: f32 = 60.0;
    let rect_width: f32 = 300.0;
    let rect_height: f32 = 40.0;

    let stats_before: Stats = player.character.stats.clone();
    let stats_after: Stats = player.character.stats.clone();

    let stat_labels: Vec<&str> = vec!["Fuerza", "Agilidad", "Vitalidad", "Resistencia"];
    let stat_values_before: Vec<f32> = vec![
        stats_before.strength,
        stats_before.agility,
        stats_before.vitality,
        stats_before.resistance,
    ];
    let stat_values_after: Vec<f32> = vec![
        stats_after.strength,
        stats_after.agility,
        stats_after.vitality,
        stats_after.resistance,
    ];

    for (i, label) in stat_labels.iter().enumerate() {
        let y: f32 = 120.0 + i as f32 * line_height;
        let stat_rect = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(left_panel_x, y, rect_width, rect_height),
            Color::new(1.0, 1.0, 1.0, 0.5),
        )
        .unwrap();
        canvas.draw(&stat_rect, DrawParam::default());

        // Dibuja el texto
        let text = Text::new(format!(
            "{}: {:.0} => {:.0}",
            label, stat_values_before[i], stat_values_after[i]
        ));
        let params = DrawParam::new().dest(Point2 {
            x: left_panel_x + 10.0,
            y: y + 5.0,
        });
        canvas.draw(&text, params);
    }
}

fn draw_continue_button(ctx: &mut Context) {}
