use crate::{
    character::{
        animation::Animation,
        animations_builder::barbarian_animations::{
            create_barbarian_attack_animation, create_barbarian_jump_animation,
            create_barbarian_move_animation,
        },
        box_collision::BoxCollision,
        character::Character,
        character_types::CharacterTypes,
        collisions_builder::{
            create_barbarian_body_collision, create_barbarian_weapon_collision,
            create_generic_foot_collision,
        },
        stats::Stats,
    },
    geometry::{position::Position, size::Size},
};

pub fn build_barbarian() -> Character {
    let position: Position = Position::new(800.0, 700.0);
    let size: Size = Size::new(210.0, 210.0);

    let vitality: f32 = 10.0;
    let strength: f32 = 10.0;
    let agility: f32 = 10.0;
    let resistance: f32 = 10.0;
    let stats: Stats = Stats::new(vitality, strength, agility, resistance);

    let character_type: CharacterTypes = CharacterTypes::Player;
    let move_animation: Animation = create_barbarian_move_animation();
    let attack_animation: Animation = create_barbarian_attack_animation();
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
        attack_animation,
        jump_animation,
        body_collision,
        foot_collision,
        weapon_collision_template,
    )
}
