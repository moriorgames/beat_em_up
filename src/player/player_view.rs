use crate::geometry::position::Position;
use crate::geometry::rectangle::rectangle::draw_solid_rectangle;
use crate::geometry::size::Size;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};

pub struct PlayerView {}

impl PlayerView {
    pub fn new() -> Self {
        PlayerView {}
    }

    pub fn draw(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        position: Position,
        size: Size,
    ) -> GameResult {
        self.draw_player(gfx, canvas, position, size);

        Ok(())
    }

    fn draw_player(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        position: Position,
        size: Size,
    ) {
        let x: f32 = position.x - size.w / 2.0;
        let y: f32 = position.y - size.h / 2.0;
        let position: Position = Position::new(x, y);
        let color: Color = Color::YELLOW;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);
    }
}
