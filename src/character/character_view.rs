pub mod character_view {
    use crate::character::character::Character;
    use crate::character::character_types::Facing;
    use crate::geometry::position::Position;
    use crate::geometry::rectangle::rectangle::{draw_solid_rectangle, draw_stroke_rectangle};
    use crate::geometry::size::Size;
    use crate::sprite::sprite_repository::SpriteRepository;
    use ggez::graphics::{Canvas, Color, DrawParam};
    use ggez::mint::Point2;
    use ggez::{Context, GameResult};

    const HEALTH_BAR_HEIGHT: f32 = 7.0;
    const HITBOX_DEBUG: bool = true;

    pub fn draw_characters(
        gfx: &mut Context,
        canvas: &mut Canvas,
        characters: &Vec<Character>,
        sprite_repository: &SpriteRepository,
    ) -> GameResult {
        for character in characters {
            draw_character(gfx, canvas, character, &sprite_repository);
            draw_health_bar(gfx, canvas, character);
        }

        Ok(())
    }

    fn draw_character( gfx: &mut Context, canvas: &mut Canvas, character: &Character, sprite_repository: &SpriteRepository,) {
        draw_generic_character(gfx, canvas, character, &sprite_repository);
    }

    fn draw_generic_character(
        gfx: &mut Context,
        canvas: &mut Canvas,
        character: &Character,
        sprite_repository: &SpriteRepository,
    ) {
        let sprite_id: String = character.get_sprite_name();
        if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
            let x: f32 = match character.facing {
                Facing::Left => character.position.x + character.size.w / 2.0,
                Facing::Right => character.position.x - character.size.w / 2.0,
            };
            let y: f32 = character.position.y - character.size.h / 2.0;
            let dst: Point2<f32> = Point2 { x, y };
            let scale: Point2<f32> = match character.facing {
                Facing::Left => Point2 { x: -2f32, y: 2f32 },
                Facing::Right => Point2 { x: 2f32, y: 2f32 },
            };

            canvas.draw(sprite, DrawParam::new().dest(dst).scale(scale));
        }

        if HITBOX_DEBUG {
            let color: Color = Color::RED;
            let x: f32 = character.position.x - character.size.w / 2.0;
            let y: f32 = character.position.y - character.size.h / 2.0;
            let position: Position = Position::new(x, y);
            let w: f32 = character.size.w;
            let h: f32 = character.size.h;
            let size: Size = Size::new(w, h);
            draw_stroke_rectangle(gfx, canvas, &position, &size, color);
        }
    }

    fn draw_health_bar(gfx: &mut Context, canvas: &mut Canvas, character: &Character) {
        let x: f32 = character.position.x - character.size.w / 2.0;
        let y: f32 = character.position.y - character.size.h / 2.0 - HEALTH_BAR_HEIGHT;
        let position: Position = Position::new(x, y);

        let health_percentage: f32 = character.current_health as f32 / character.max_health as f32;
        let w: f32 = character.size.w * health_percentage;
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        let color: Color = Color::RED;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);

        let color: Color = Color::BLACK;
        let w: f32 = character.size.w;
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        draw_stroke_rectangle(gfx, canvas, &position, &size, color);
    }
}
