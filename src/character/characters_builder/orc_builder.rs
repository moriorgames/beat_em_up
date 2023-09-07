use crate::{
    character::{
        animation::Animation,
        animations_builder::orc_animations::{
            create_orc_fast_attack_animation, create_orc_jump_animation, create_orc_move_animation,
        },
        box_collision::BoxCollision,
        character::Character,
        character_types::CharacterTypes,
        collisions_builder::{
            create_generic_foot_collision, create_orc_body_collision,
            create_orc_weapon_collision_template,
        },
        stats::Stats,
    },
    geometry::{position::Position, size::Size},
};

pub fn build_orc(position: Position) -> Character {
    let size: Size = Size::new(210.0, 210.0);

    let vitality: f32 = 5.0;
    let strength: f32 = 8.0;
    let agility: f32 = 6.0;
    let resistance: f32 = 5.0;
    let stats: Stats = Stats::new(vitality, strength, agility, resistance);

    let character_type: CharacterTypes = CharacterTypes::Enemy;
    let move_animation: Animation = create_orc_move_animation();
    let fast_attack_animation: Animation = create_orc_fast_attack_animation();
    let jump_animation: Animation = create_orc_jump_animation();
    let body_collision: BoxCollision = create_orc_body_collision(size.w);
    let foot_collision: BoxCollision = create_generic_foot_collision();
    let weapon_collision_template: BoxCollision = create_orc_weapon_collision_template();

    Character::new(
        position,
        size,
        stats,
        character_type,
        move_animation,
        fast_attack_animation,
        jump_animation,
        body_collision,
        foot_collision,
        weapon_collision_template,
    )
}
