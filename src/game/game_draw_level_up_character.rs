use crate::sprite::sprite_repository::SpriteRepository;
use ggez::{
    graphics::{Canvas, DrawParam},
    mint::Point2,
};

pub fn draw_level_up_character(canvas: &mut Canvas, sprite_repository: &SpriteRepository) {
    let sprite_id: String = "barbarian_level_up".to_string();
    if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
        let x: f32 = 150.0;
        let y: f32 = 250.0;
        let dst: Point2<f32> = Point2 { x, y };
        let scale: Point2<f32> = Point2 { x: 3f32, y: 3f32 };

        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale);
        canvas.draw(sprite, draw_params);
    }
}
