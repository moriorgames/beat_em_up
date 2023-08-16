pub mod character_view {
    use crate::geometry::rectangle::rectangle::draw_stroke_rectangle;
    use crate::sprite::sprite_repository::SpriteRepository;
    use crate::world::world::World;
    use ggez::graphics::{Canvas, Color, DrawParam};
    use ggez::mint::Point2;
    use ggez::{GameResult, Context};

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
            draw_stroke_rectangle(gfx, canvas, &world.position, &world.size, color);
        }

        Ok(())
    }
}
