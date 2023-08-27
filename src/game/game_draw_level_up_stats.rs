use super::level_up_panel_config::LevelUpPanelConfig;
use crate::{character::character_stats::Stats, player::player::Player};
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context,
};

pub fn draw_character_stats(gfx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
    let config: LevelUpPanelConfig = LevelUpPanelConfig::new();
    let left_top_panel: Rect = config.left_top_subpanel;

    let level: u32 = player.level;
    let points_available: u32 = player.experience;

    let top_info_y: f32 = left_top_panel.y + config.padding;
    let text_level: Text = Text::new(format!("Nivel: {}", level));
    let text_points: Text = Text::new(format!("Puntos disponibles: {}", points_available));

    let level_params: DrawParam = DrawParam::new().dest(Point2 {
        x: left_top_panel.x + config.padding,
        y: top_info_y,
    });
    canvas.draw(&text_level, level_params);

    let points_params: DrawParam = DrawParam::new().dest(Point2 {
        x: left_top_panel.x + config.padding,
        y: top_info_y + config.padding * 2.0,
    });
    canvas.draw(&text_points, points_params);

    let line_y: f32 = top_info_y + config.padding * 4.0;
    let line: Mesh = Mesh::new_line(
        gfx,
        &[
            Point2 {
                x: left_top_panel.x + config.padding,
                y: line_y,
            },
            Point2 {
                x: left_top_panel.x + left_top_panel.w - config.padding,
                y: line_y,
            },
        ],
        2.0,
        Color::new(1.0, 1.0, 1.0, 0.5),
    )
    .unwrap();

    canvas.draw(&line, DrawParam::default());

    let left_bot_panel: Rect = config.left_bot_subpanel;

    let line_height: f32 = 60.0;
    let rect_width: f32 = left_bot_panel.w - config.padding * 2.0;
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
        let y: f32 = left_bot_panel.y + config.padding + i as f32 * line_height;
        let stat_rect: Mesh = Mesh::new_rectangle(
            gfx,
            DrawMode::stroke(2.0),
            Rect::new(
                left_bot_panel.x + config.padding,
                y,
                rect_width,
                rect_height,
            ),
            Color::new(1.0, 1.0, 1.0, 0.5),
        )
        .unwrap();
        canvas.draw(&stat_rect, DrawParam::default());

        let text: Text = Text::new(format!(
            "{}: {:.0} => {:.0}",
            label, stat_values_before[i], stat_values_after[i]
        ));
        let params: DrawParam = DrawParam::new().dest(Point2 {
            x: left_bot_panel.x + config.padding + 10.0,
            y: y + 15.0,
        });
        canvas.draw(&text, params);
    }
}
