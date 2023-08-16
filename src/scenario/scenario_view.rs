pub mod character_view {
    use crate::sprite::sprite_repository::SpriteRepository;
    use ggez::graphics::{Canvas, DrawParam};
    use ggez::mint::Point2;
    use ggez::GameResult;

    pub fn draw_scenario(canvas: &mut Canvas, sprite_repository: &SpriteRepository) -> GameResult {
        let sprite_id: String = "scenario".to_string();

        if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
            let x: f32 = 0.0;
            let y: f32 = 0.0;
            let dst: Point2<f32> = Point2 { x, y };
            let scale: Point2<f32> = Point2 { x: 2.0, y: 2.0 };

            canvas.draw(sprite, DrawParam::new().dest(dst).scale(scale));
        }

        Ok(())
    }
}
