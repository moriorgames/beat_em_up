use crate::sprite::sprite_repository::SpriteRepository;
use ggez::{
    graphics::{Canvas, Color, DrawParam, Image},
    mint::Point2,
};

pub fn draw_fire(canvas: &mut Canvas, sprite_repository: &SpriteRepository, frame_count: u128) {
    let frame_index: u128 = (frame_count / 3) % 8;
    let sprite_id: String = format!("fire_{}", frame_index);
    if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
        let x: f32 = 340.0;
        let y: f32 = 300.0;
        let dst: Point2<f32> = Point2 { x, y };
        let scale: Point2<f32> = Point2 { x: 2f32, y: 2f32 };

        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale);
        canvas.draw(sprite, draw_params);

        let dst: Point2<f32> = Point2 { x: 0.0, y: 0.0 };
        let color: Color = Color::new(1.0, 1.0, 1.0, 0.80);
        let mask_image: &Image = sprite_repository.get_sprite("fire_mask").unwrap();
        let draw_params: DrawParam = DrawParam::new().dest(dst).scale(scale).color(color);
        canvas.draw(mask_image, draw_params);
    }
}
