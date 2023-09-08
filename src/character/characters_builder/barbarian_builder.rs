use crate::{
    character::{
        animation::Animation, animations_builder::barbarian_animations::*,
        box_collision::BoxCollision, character::Character, character_types::CharacterTypes,
        collisions_builder::*, stats::Stats,
    },
    geometry::{position::Position, size::Size},
};

pub fn build_barbarian(position: Position) -> Character {
    let size: Size = Size::new(210.0, 210.0);

    let vitality: f32 = 10.0;
    let strength: f32 = 10.0;
    let agility: f32 = 10.0;
    let resistance: f32 = 10.0;
    let stats: Stats = Stats::new(vitality, strength, agility, resistance);

    let character_type: CharacterTypes = CharacterTypes::Player;
    let move_animation: Animation = create_barbarian_move_animation();
    let fast_attack_animation: Animation = create_barbarian_fast_attack_animation();
    let slow_attack_animation: Animation = create_barbarian_slow_attack_animation();
    let jump_animation: Animation = create_barbarian_jump_animation();
    let body_collision: BoxCollision = create_barbarian_body_collision(size.w);
    let foot_collision: BoxCollision = create_generic_foot_collision();
    let weapon_collision_template: BoxCollision = create_barbarian_weapon_collision();

    Character::new(
        position,
        size,
        stats,
        character_type,
        move_animation,
        fast_attack_animation,
        slow_attack_animation,
        jump_animation,
        body_collision,
        foot_collision,
        weapon_collision_template,
    )
}
