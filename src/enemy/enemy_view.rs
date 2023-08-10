use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};
use nalgebra::Point2;

pub struct EnemyView;

impl EnemyView {
    const WIDTH: f32 = 20.0;
    const HEIGHT: f32 = 20.0;

    pub fn new() -> Self {
        EnemyView {}
    }

    pub fn draw(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        position: Point2<f32>,
    ) -> GameResult {
        let mode: DrawMode = DrawMode::fill();
        let bounds: Rect = Rect::new(
            position.x - Self::WIDTH / 2.0,
            position.y - Self::HEIGHT / 2.0,
            Self::WIDTH,
            Self::HEIGHT,
        );
        let color = Color::RED;
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color)?;
        let param: DrawParam = DrawParam::new();

        canvas.draw(&drawable, param);

        Ok(())
    }
}
