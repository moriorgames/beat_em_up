pub mod character_view {
    use crate::geometry::position::Position;
    use crate::geometry::rectangle::rectangle::draw_stroke_rectangle;
    use crate::geometry::size::Size;
    use crate::sprite::sprite_repository::SpriteRepository;
    use crate::world::world::World;
    use ggez::graphics::{Canvas, Color, DrawParam};
    use ggez::mint::Point2;
    use ggez::{Context, GameResult};

    const HITBOX_DEBUG: bool = true;

    pub fn draw(
        gfx: &mut Context,
        canvas: &mut Canvas,
        world: &World,
        sprite_repository: &SpriteRepository,
    ) -> GameResult {
        let sprite_id: String = "world".to_string();

        if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
            let x: f32 = 0.0;
            let y: f32 = 0.0;
            let dst: Point2<f32> = Point2 { x, y };
            let scale: Point2<f32> = Point2 { x: 2.0, y: 2.0 };

            canvas.draw(sprite, DrawParam::new().dest(dst).scale(scale));
        }

        if HITBOX_DEBUG {
            let color: Color = Color::RED;
            let position: Position = Position::new(world.bounds.x, world.bounds.y);
            let size: Size = Size::new(world.bounds.w, world.bounds.h);
            draw_stroke_rectangle(gfx, canvas, &position, &size, color);
        }

        Ok(())
    }
}
