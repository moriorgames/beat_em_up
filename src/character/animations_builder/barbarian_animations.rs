use crate::character::animation::Animation;

pub fn create_barbarian_move_animation() -> Animation {
    let sprite: String = "barbarian".to_string();
    let action_type: String = "move".to_string();
    let move_frames: u8 = 8;
    let delay: u8 = 4;

    Animation::new(sprite, action_type, move_frames, delay)
}

pub fn create_barbarian_fast_attack_animation() -> Animation {
    let sprite: String = "barbarian".to_string();
    let action_type: String = "attack".to_string();
    let move_frames: u8 = 5;
    let delay: u8 = 7;

    Animation::new(sprite, action_type, move_frames, delay)
}

pub fn create_barbarian_slow_attack_animation() -> Animation {
    let sprite: String = "barbarian".to_string();
    let action_type: String = "attack".to_string();
    let move_frames: u8 = 5;
    let delay: u8 = 11;

    Animation::new(sprite, action_type, move_frames, delay)
}

pub fn create_barbarian_jump_animation() -> Animation {
    let sprite: String = "barbarian".to_string();
    let action_type: String = "jump".to_string();
    let move_frames: u8 = 3;
    let delay: u8 = 7;

    Animation::new(sprite, action_type, move_frames, delay)
}
