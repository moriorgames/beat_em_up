pub mod character_view {
    use crate::character::character::Character;
    use crate::character::character_types::CharacterTypes;
    use crate::geometry::position::Position;
    use crate::geometry::rectangle::rectangle::{draw_solid_rectangle, draw_stroke_rectangle};
    use crate::geometry::size::Size;
    use ggez::graphics::{Canvas, Color};
    use ggez::{Context, GameResult};

    const HEALTH_BAR_HEIGHT: f32 = 7.0;
    const HEALTH_BAR_Y_OFFSET: f32 = -25.0;

    pub fn draw_characters(
        gfx: &mut Context,
        canvas: &mut Canvas,
        characters: Vec<Character>,
    ) -> GameResult {
        for character in characters {
            draw_character(gfx, canvas, character.clone());
            draw_health_bar(gfx, canvas, character.clone());
        }

        Ok(())
    }

    fn draw_character(gfx: &mut Context, canvas: &mut Canvas, character: Character) {
        match character.character_type {
            CharacterTypes::Player => draw_player_character(gfx, canvas, character.clone()),
            _ => draw_generic_character(gfx, canvas, character.clone()),
        }
    }

    fn draw_player_character(gfx: &mut Context, canvas: &mut Canvas, character: Character) {
        let size: Size = character.size;
        let x: f32 = character.position.x - size.w / 2.0;
        let y: f32 = character.position.y - size.h / 2.0;
        let position: Position = Position::new(x, y);
        let color: Color = Color::YELLOW;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);
    }

    fn draw_generic_character(gfx: &mut Context, canvas: &mut Canvas, character: Character) {
        let size: Size = character.size;
        let x: f32 = character.position.x - size.w / 2.0;
        let y: f32 = character.position.y - size.h / 2.0;
        let position: Position = Position::new(x, y);
        let color: Color = Color::GREEN;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);
    }

    fn draw_health_bar(gfx: &mut Context, canvas: &mut Canvas, character: Character) {
        let enemy_size: Size = character.size;
        let x: f32 = character.position.x - enemy_size.w / 2.0;
        let y: f32 = character.position.y + HEALTH_BAR_Y_OFFSET;
        let position: Position = Position::new(x, y);

        let health_percentage: f32 = character.current_health as f32 / character.max_health as f32;
        let w: f32 = enemy_size.w * health_percentage;
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        let color: Color = Color::RED;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);

        let color: Color = Color::BLACK;
        let w: f32 = enemy_size.w;
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        draw_stroke_rectangle(gfx, canvas, &position, &size, color);
    }
}
