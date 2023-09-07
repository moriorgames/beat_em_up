use crate::character::animation::Animation;

pub fn create_orc_move_animation() -> Animation {
    let sprite: String = "orc".to_string();
    let action_type: String = "move".to_string();
    let move_frames: u8 = 6;
    let delay: u8 = 6;

    Animation::new(sprite, action_type, move_frames, delay)
}

pub fn create_orc_fast_attack_animation() -> Animation {
    let sprite: String = "orc".to_string();
    let action_type: String = "attack".to_string();
    let move_frames: u8 = 5;
    let delay: u8 = 11;

    Animation::new(sprite, action_type, move_frames, delay)
}

pub fn create_orc_jump_animation() -> Animation {
    let sprite: String = "orc".to_string();
    let action_type: String = "jump".to_string();
    let move_frames: u8 = 3;
    let delay: u8 = 7;

    Animation::new(sprite, action_type, move_frames, delay)
}
