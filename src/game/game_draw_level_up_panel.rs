use super::level_up_panel_config::LevelUpPanelConfig;
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    Context,
};

pub fn draw_background_panel(gfx: &mut Context, canvas: &mut Canvas) {
    let config: LevelUpPanelConfig = LevelUpPanelConfig::new();
    draw_main_panel(gfx, canvas, config.main_panel);

    let border_mode: DrawMode = DrawMode::stroke(2.0);
    let border_color: Color = Color::new(0.7, 0.7, 0.7, 1.0);

    let left_panel: Mesh =
        Mesh::new_rectangle(gfx, border_mode, config.left_subpanel, border_color).unwrap();
    canvas.draw(&left_panel, DrawParam::default());

    let right_panel: Mesh =
        Mesh::new_rectangle(gfx, border_mode, config.right_subpanel, border_color).unwrap();
    canvas.draw(&right_panel, DrawParam::default());
}

fn draw_main_panel(gfx: &mut Context, canvas: &mut Canvas, bg_bounds: Rect) {
    let bg_mode: DrawMode = DrawMode::fill();
    let bg_color: Color = Color::new(0.1, 0.1, 0.1, 1.0);
    let bg_draw_params: DrawParam = DrawParam::new();
    let bg_panel: Mesh = Mesh::new_rectangle(gfx, bg_mode, bg_bounds, bg_color).unwrap();
    canvas.draw(&bg_panel, bg_draw_params);
}
