use super::enemy_transformation::EnemyTransformation;
use crate::geometry::position::Position;
use crate::geometry::rectangle::rectangle::{draw_solid_rectangle, draw_stroke_rectangle};
use crate::geometry::size::Size;
use ggez::graphics::{Canvas, Color};
use ggez::{Context, GameResult};

pub struct EnemyView;

impl EnemyView {
    const HEALTH_BAR_HEIGHT: f32 = 7.0;
    const HEALTH_BAR_Y_OFFSET: f32 = -25.0;

    pub fn new() -> Self {
        EnemyView {}
    }

    pub fn draw(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        enemy_transformation: EnemyTransformation,
    ) -> GameResult {
        self.draw_enemy(gfx, canvas, enemy_transformation.clone());
        self.draw_health_bar(gfx, canvas, enemy_transformation.clone());

        Ok(())
    }

    fn draw_enemy(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        enemy_transformation: EnemyTransformation,
    ) {
        let size: Size = enemy_transformation.size;
        let x: f32 = enemy_transformation.position.x - size.w / 2.0;
        let y: f32 = enemy_transformation.position.y - size.h / 2.0;
        let position: Position = Position::new(x, y);
        let color: Color = Color::GREEN;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);
    }

    fn draw_health_bar(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        enemy_transformation: EnemyTransformation,
    ) {
        let enemy_size: Size = enemy_transformation.size;
        let x: f32 = enemy_transformation.position.x - enemy_size.w / 2.0;
        let y: f32 = enemy_transformation.position.y + Self::HEALTH_BAR_Y_OFFSET;
        let position: Position = Position::new(x, y);

        let w: f32 = enemy_size.w * enemy_transformation.health_percentage;
        let h: f32 = Self::HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        let color: Color = Color::RED;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);

        let color: Color = Color::BLACK;
        let w: f32 = enemy_size.w;
        let h: f32 = Self::HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        draw_stroke_rectangle(gfx, canvas, &position, &size, color);
    }
}
