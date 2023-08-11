use super::enemy_transformation::EnemyTransformation;
use crate::geometry::position::Position;
use crate::geometry::size::Size;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
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
        let mode: DrawMode = DrawMode::fill();
        let position: Position = enemy_transformation.position;
        let size: Size = enemy_transformation.size;
        let bounds: Rect = Rect::new(
            position.x - size.width / 2.0,
            position.y - size.height / 2.0,
            size.width,
            size.height,
        );
        let color: Color = Color::GREEN;
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color).unwrap();
        let param: DrawParam = DrawParam::new();

        canvas.draw(&drawable, param);
    }

    fn draw_health_bar(
        &mut self,
        gfx: &mut Context,
        canvas: &mut Canvas,
        enemy_transformation: EnemyTransformation,
    ) {
        let position: Position = enemy_transformation.position;
        let size: Size = enemy_transformation.size;

        let mode: DrawMode = DrawMode::fill();
        let current_health_width: f32 = size.width * enemy_transformation.health_percentage;
        let health_rect = Rect::new(
            position.x - size.width / 2.0,
            position.y + Self::HEALTH_BAR_Y_OFFSET,
            current_health_width,
            Self::HEALTH_BAR_HEIGHT,
        );
        let color: Color = Color::RED;
        let health_drawable = Mesh::new_rectangle(gfx, mode, health_rect, color).unwrap();
        let param: DrawParam = DrawParam::new();
        canvas.draw(&health_drawable, param);

        let mode: DrawMode = DrawMode::stroke(1.5);
        let bounds: Rect = Rect::new(
            position.x - size.width / 2.0,
            position.y + Self::HEALTH_BAR_Y_OFFSET,
            size.width,
            Self::HEALTH_BAR_HEIGHT,
        );
        let color: Color = Color::BLACK;
        let drawable: Mesh = Mesh::new_rectangle(gfx, mode, bounds, color).unwrap();
        let param: DrawParam = DrawParam::new();
        canvas.draw(&drawable, param);
    }
}
