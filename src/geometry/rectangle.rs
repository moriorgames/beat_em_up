pub mod rectangle {
    use crate::geometry::position::Position;
    use crate::geometry::size::Size;
    use ggez::{
        graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
        Context,
    };

    pub fn draw_solid_rectangle(
        gfx: &mut Context,
        canvas: &mut Canvas,
        position: &Position,
        size: &Size,
        color: Color,
    ) {
        let mode: DrawMode = DrawMode::fill();
        let bounds: Rect = Rect::new(position.x, position.y, size.w, size.h);
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color).unwrap();
        let param: DrawParam = DrawParam::new();

        canvas.draw(&drawable, param);
    }

    pub fn draw_stroke_rectangle(
        gfx: &mut Context,
        canvas: &mut Canvas,
        position: &Position,
        size: &Size,
        color: Color,
    ) {
        let mode: DrawMode = DrawMode::stroke(2.0);
        let bounds: Rect = Rect::new(position.x, position.y, size.w, size.h);
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color).unwrap();
        let param: DrawParam = DrawParam::new();

        canvas.draw(&drawable, param);
    }
}
