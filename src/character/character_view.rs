pub mod character_view {
    use crate::character::character::Character;
    use crate::character::character_types::CharacterTypes;
    use crate::geometry::position::Position;
    use crate::geometry::rectangle::rectangle::{draw_solid_rectangle, draw_stroke_rectangle};
    use crate::geometry::size::Size;
    use crate::sprite::sprite_repository::SpriteRepository;
    use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
    use ggez::mint::Point2;
    use ggez::{Context, GameResult};

    const HEALTH_BAR_HEIGHT: f32 = 7.0;
    const HITBOX_DEBUG: bool = true;

    const SKULL_WHITE: Color = Color::new(230.0 / 255.0, 230.0 / 255.0, 230.0 / 255.0, 1.0);

    const MATRIX_LEN: usize = 16;

    const SKULL_PIXELS: [[u8; MATRIX_LEN]; MATRIX_LEN] = [
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 0],
        [0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
        [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
        [1, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 1, 0],
        [1, 2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1],
        [1, 2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1],
        [1, 2, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 1],
        [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1],
        [0, 1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 1, 0],
        [0, 0, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 0, 0],
        [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 2, 1, 2, 2, 1, 2, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    ];

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

    fn draw_character(
        gfx: &mut Context,
        canvas: &mut Canvas,
        character: &Character,
        sprite_repository: &SpriteRepository,
    ) {
        match character.character_type {
            CharacterTypes::Player => {
                draw_player_character(gfx, canvas, character, &sprite_repository)
            }
            _ => draw_generic_character(gfx, canvas, character, &sprite_repository),
        };
    }

    fn draw_player_character(
        gfx: &mut Context,
        canvas: &mut Canvas,
        character: &Character,
        sprite_repository: &SpriteRepository,
    ) {
        let sprite_id: String = character.get_sprite_name();
        if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
            let x: f32 = character.position.x - character.size.w / 2.0;
            let y: f32 = character.position.y - character.size.h / 2.0;
            let dst: Point2<f32> = Point2 { x, y };
            let scale: Point2<f32> = Point2 { x: 2.0, y: 2.0 };
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

    fn draw_generic_character(
        gfx: &mut Context,
        canvas: &mut Canvas,
        character: &Character,
        sprite_repository: &SpriteRepository,
    ) {
        let sprite_id: String = character.get_sprite_name();
        if let Some(sprite) = sprite_repository.get_sprite(&sprite_id) {
            let x: f32 = character.position.x - character.size.w / 2.0;
            let y: f32 = character.position.y - character.size.h / 2.0;
            let dst: Point2<f32> = Point2 { x, y };
            let scale: Point2<f32> = Point2 { x: 2.0, y: 2.0 };
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
