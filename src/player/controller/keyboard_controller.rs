pub mod keyboard_controller {
    use crate::player::player_controls::PlayerIntention;
    use ggez::{input::keyboard::KeyCode, Context};

    pub fn input(ctx: &Context) -> PlayerIntention {
        let pressed_keys: &std::collections::HashSet<KeyCode> = ctx.keyboard.pressed_keys();
        PlayerIntention {
            move_left: pressed_keys.contains(&KeyCode::Left),
            move_right: pressed_keys.contains(&KeyCode::Right),
            move_up: pressed_keys.contains(&KeyCode::Up),
            move_down: pressed_keys.contains(&KeyCode::Down),
            quit: pressed_keys.contains(&KeyCode::Escape),
        }
    }
}
