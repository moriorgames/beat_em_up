use super::level_up_panel_config::LevelUpPanelConfig;
use crate::player::player::Player;
use ggez::{
    graphics::{Canvas, Color, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context,
};

pub fn draw_character_info(gfx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
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
}
