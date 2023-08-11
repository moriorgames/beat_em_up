use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};
use nalgebra::Point2;

pub struct PlayerView {}

impl PlayerView {
    const WIDTH: f32 = 55.0;
    const HEIGHT: f32 = 45.0;

    pub fn new() -> Self {
        PlayerView {}
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
        let color = Color::YELLOW;
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color)?;
        let param: DrawParam = DrawParam::new();

        canvas.draw(&drawable, param);

        Ok(())
    }
}
