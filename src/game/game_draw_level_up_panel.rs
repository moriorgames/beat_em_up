use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    Context,
};

pub fn draw_background_panel(ctx: &mut Context, canvas: &mut Canvas) {
    let bg_bounds: Rect = Rect::new(800.0, 100.0, 900.0, 800.0);
    draw_main_panel(ctx, canvas, bg_bounds);

    let padding: f32 = 20.0;
    let subpanel_width: f32 = (bg_bounds.w - 3.0 * padding) / 2.0;
    let subpanel_height: f32 = bg_bounds.h - 2.0 * padding;

    let border_mode: DrawMode = DrawMode::stroke(2.0);
    let border_color: Color = Color::new(0.7, 0.7, 0.7, 1.0);

    let left_bounds: Rect = Rect::new(
        bg_bounds.x + padding,
        bg_bounds.y + padding,
        subpanel_width,
        subpanel_height,
    );
    let left_panel: Mesh =
        Mesh::new_rectangle(ctx, border_mode, left_bounds, border_color).unwrap();
    canvas.draw(&left_panel, DrawParam::default());

    let right_bounds: Rect = Rect::new(
        bg_bounds.x + 2.0 * padding + subpanel_width,
        bg_bounds.y + padding,
        subpanel_width,
        subpanel_height,
    );
    let right_panel: Mesh =
        Mesh::new_rectangle(ctx, border_mode, right_bounds, border_color).unwrap();
    canvas.draw(&right_panel, DrawParam::default());
}

fn draw_main_panel(ctx: &mut Context, canvas: &mut Canvas, bg_bounds: Rect) {
    let bg_mode: DrawMode = DrawMode::fill();
    let bg_color: Color = Color::new(0.1, 0.1, 0.1, 1.0);
    let bg_draw_params: DrawParam = DrawParam::new();
    let bg_panel: Mesh = Mesh::new_rectangle(ctx, bg_mode, bg_bounds, bg_color).unwrap();
    canvas.draw(&bg_panel, bg_draw_params);
}
