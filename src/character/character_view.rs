pub mod character_view {
    use std::cmp::Ordering;

    use crate::character::box_collision::BoxCollision;
    use crate::character::character::Character;
    use crate::character::character_types::Facing;
    use crate::geometry::position::Position;
    use crate::geometry::rectangle::rectangle::{draw_solid_rectangle, draw_stroke_rectangle};
    use crate::geometry::size::Size;
    use crate::sprite::sprite_repository::SpriteRepository;
    use ggez::graphics::{Canvas, Color, DrawParam};
    use ggez::mint::Point2;
    use ggez::{Context, GameResult};

    const HEALTH_BAR_HEIGHT: f32 = 8.0;
    const HEALTH_BAR_FACTOR: f32 = 100.0;
    const HITBOX_DEBUG: bool = true;

    pub fn draw_characters(
        gfx: &mut Context,
        canvas: &mut Canvas,
        characters: &Vec<Character>,
        sprite_repository: &SpriteRepository,
    ) -> GameResult {
        for character in get_ordered_characters_by_position(characters) {
            draw_character(gfx, canvas, &character, &sprite_repository);
            draw_bars(gfx, canvas, &character);
        }

        Ok(())
    }

    fn draw_character(
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
                Facing::Left => Point2 { x: -3f32, y: 3f32 },
                Facing::Right => Point2 { x: 3f32, y: 3f32 },
            };

            canvas.draw(sprite, DrawParam::new().dest(dst).scale(scale));
        }

        if HITBOX_DEBUG {
            let foot_collision: BoxCollision = character.foot_collision_to_world_space();
            let color: Color = Color::GREEN;
            let x: f32 = foot_collision.x;
            let y: f32 = foot_collision.y;
            let position: Position = Position::new(x, y);
            let w: f32 = foot_collision.w;
            let h: f32 = foot_collision.h;
            let size: Size = Size::new(w, h);
            draw_stroke_rectangle(gfx, canvas, &position, &size, color);

            let body_collision: BoxCollision = character.body_collision_to_world_space();
            let color: Color = Color::RED;
            let x: f32 = body_collision.x;
            let y: f32 = body_collision.y;
            let position: Position = Position::new(x, y);
            let w: f32 = body_collision.w;
            let h: f32 = body_collision.h;
            let size: Size = Size::new(w, h);
            draw_stroke_rectangle(gfx, canvas, &position, &size, color);

            let weapon_collision: BoxCollision = character.weapon_collision_to_world_space();
            let color: Color = Color::BLACK;
            let x: f32 = weapon_collision.x;
            let y: f32 = weapon_collision.y;
            let position: Position = Position::new(x, y);
            let w: f32 = weapon_collision.w;
            let h: f32 = weapon_collision.h;
            let size: Size = Size::new(w, h);
            draw_stroke_rectangle(gfx, canvas, &position, &size, color);
        }
    }

    fn draw_bars(gfx: &mut Context, canvas: &mut Canvas, character: &Character) {
        let x: f32 = character.position.x - (character.size.w - (HEALTH_BAR_FACTOR)) / 2.0;
        let y: f32 = character.position.y - character.size.h / 2.0 - HEALTH_BAR_HEIGHT;
        let position: Position = Position::new(x, y);

        let w: f32 = character.size.w - (HEALTH_BAR_FACTOR);
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        let color: Color = Color::RED;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);

        let x: f32 = character.position.x - (character.size.w - (HEALTH_BAR_FACTOR)) / 2.0 - 1.5;
        let y: f32 = character.position.y - character.size.h / 2.0 - HEALTH_BAR_HEIGHT - 1.5;
        let position: Position = Position::new(x, y);

        let w: f32 = character.size.w - (HEALTH_BAR_FACTOR) + 1.5;
        let h: f32 = HEALTH_BAR_HEIGHT + 1.5;
        let size: Size = Size::new(w, h);
        let color: Color = Color::BLACK;
        draw_stroke_rectangle(gfx, canvas, &position, &size, color);

        let x: f32 = character.position.x - (character.size.w - (HEALTH_BAR_FACTOR)) / 2.0;
        let y: f32 = character.position.y - character.size.h / 2.0 - HEALTH_BAR_HEIGHT;
        let position: Position = Position::new(x, y);

        let health_percentage: f32 = character.current_health as f32 / character.max_health as f32;
        let w: f32 = character.size.w * health_percentage - (HEALTH_BAR_FACTOR);
        let h: f32 = HEALTH_BAR_HEIGHT;
        let size: Size = Size::new(w, h);
        let color: Color = Color::YELLOW;
        draw_solid_rectangle(gfx, canvas, &position, &size, color);
    }

    fn get_ordered_characters_by_position(characters: &Vec<Character>) -> Vec<Character> {
        let mut ordered_characters: Vec<Character> = characters.clone();
        ordered_characters.sort_by(|a, b| {
            a.position
                .y
                .partial_cmp(&b.position.y)
                .unwrap_or(Ordering::Equal)
        });
        ordered_characters
    }
}
