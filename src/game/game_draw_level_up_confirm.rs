use super::level_up_panel_config::LevelUpPanelConfig;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context,
};

pub fn draw_confirm_button(ctx: &mut Context, canvas: &mut Canvas, selected_stat: &String) {
    let config: LevelUpPanelConfig = LevelUpPanelConfig::new();
    let left_bot_panel: Rect = config.left_bot_subpanel;
    let y: f32 = left_bot_panel.y + config.padding + 4 as f32 * LevelUpPanelConfig::ROW_LINE_HEIGHT;
    let label: &str = "confirm";

    let color: Color = if label == selected_stat {
        Color::GREEN
    } else {
        Color::new(1.0, 1.0, 1.0, 0.5)
    };

    let confirm_rect: Mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::stroke(2.0),
        Rect::new(
            left_bot_panel.x + config.padding,
            y,
            left_bot_panel.w - config.padding * 2.0,
            LevelUpPanelConfig::ROW_HEIGHT,
        ),
        color,
    )
    .unwrap();

    canvas.draw(&confirm_rect, DrawParam::default());

    let text: Text = Text::new(label);

    let params: DrawParam = DrawParam::new().dest(Point2 {
        x: left_bot_panel.x + config.padding + 10.0,
        y: y + 15.0,
    });

    canvas.draw(&text, params);
}
