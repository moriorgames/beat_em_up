use super::level_up_panel_config::LevelUpPanelConfig;
use crate::{character::character_stats::Stats, player::player::Player};
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context,
};

pub fn draw_character_main_stats(gfx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
    let config: LevelUpPanelConfig = LevelUpPanelConfig::new();
    let left_bot_panel: Rect = config.left_bot_subpanel;

    let rect_width: f32 = left_bot_panel.w - config.padding * 2.0;

    let stats: Stats = player.character.stats.clone();
    let stat_labels: Vec<&str> = vec!["Vitalidad", "Fuerza", "Agilidad", "Resistencia"];
    let stat_values: Vec<f32> = vec![
        stats.vitality,
        stats.strength,
        stats.agility,
        stats.resistance,
    ];

    for (i, label) in stat_labels.iter().enumerate() {
        let y: f32 =
            left_bot_panel.y + config.padding + i as f32 * LevelUpPanelConfig::ROW_LINE_HEIGHT;
        let stat_rect: Mesh = Mesh::new_rectangle(
            gfx,
            DrawMode::stroke(2.0),
            Rect::new(
                left_bot_panel.x + config.padding,
                y,
                rect_width,
                LevelUpPanelConfig::ROW_HEIGHT,
            ),
            Color::new(1.0, 1.0, 1.0, 0.5),
        )
        .unwrap();

        canvas.draw(&stat_rect, DrawParam::default());

        let text: Text = Text::new(format!("{}: {:.0}", label, stat_values[i]));

        let params: DrawParam = DrawParam::new().dest(Point2 {
            x: left_bot_panel.x + config.padding + 10.0,
            y: y + 15.0,
        });

        canvas.draw(&text, params);
    }
}
