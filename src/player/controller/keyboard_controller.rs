pub mod keyboard_controller {
    use crate::player::{level_up_controls::LevelUpIntention, player_controls::PlayerIntention};
    use ggez::{input::keyboard::KeyCode, Context};

    pub fn input(ctx: &Context) -> PlayerIntention {
        let pressed_keys: &std::collections::HashSet<KeyCode> = ctx.keyboard.pressed_keys();
        PlayerIntention {
            move_left: pressed_keys.contains(&KeyCode::Left),
            move_right: pressed_keys.contains(&KeyCode::Right),
            move_up: pressed_keys.contains(&KeyCode::Up),
            move_down: pressed_keys.contains(&KeyCode::Down),
            attack: pressed_keys.contains(&KeyCode::A),
            jump: pressed_keys.contains(&KeyCode::S),
            quit: pressed_keys.contains(&KeyCode::Escape),
        }
    }

    pub fn level_up(ctx: &Context) -> LevelUpIntention {
        LevelUpIntention {
            move_left: ctx.keyboard.is_key_just_released(KeyCode::Left),
            move_right: ctx.keyboard.is_key_just_released(KeyCode::Right),
            move_up: ctx.keyboard.is_key_just_released(KeyCode::Up),
            move_down: ctx.keyboard.is_key_just_released(KeyCode::Down),
            confirm: ctx.keyboard.is_key_just_released(KeyCode::A),
            quit: ctx.keyboard.is_key_just_released(KeyCode::Escape),
        }
    }
}
