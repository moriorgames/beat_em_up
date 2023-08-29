pub mod character_builder {
    use crate::{
        character::{
            animation::Animation,
            animations_builder::{
                barbarian_animations::{
                    create_barbarian_attack_animation, create_barbarian_jump_animation,
                    create_barbarian_move_animation,
                },
                orc_animations::{
                    create_orc_attack_animation, create_orc_jump_animation,
                    create_orc_move_animation,
                },
            },
            box_collision::BoxCollision,
            character::Character,
            character_stats::Stats,
            character_types::CharacterTypes,
            collisions_builder::{
                create_barbarian_body_collision, create_barbarian_weapon_collision,
                create_generic_foot_collision, create_orc_body_collision,
                create_orc_weapon_collision_template,
            },
        },
        geometry::{position::Position, size::Size},
    };

    pub fn build_player() -> Character {
        let position: Position = Position::new(800.0, 700.0);
        let size: Size = Size::new(210.0, 210.0);

        let vitality: f32 = 5.0;
        let strength: f32 = 5.0;
        let agility: f32 = 5.0;
        let resistance: f32 = 5.0;
        let stats: Stats = Stats::new(vitality, strength, agility, resistance);

        let character_type: CharacterTypes = CharacterTypes::Player;
        let move_animation: Animation = create_barbarian_move_animation();
        let attack_animation: Animation = create_barbarian_attack_animation();
        let jump_animation: Animation = create_barbarian_jump_animation();
        let body_collision: BoxCollision = create_barbarian_body_collision(size.w);
        let foot_collision: BoxCollision = create_generic_foot_collision();
        let weapon_collision_template: BoxCollision = create_barbarian_weapon_collision();
        let player: Character = Character::new(
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
        );

        let debug: Character = player.clone();
        println!(
            "- player {:?} speed {}, speed_jump {}, damage {}, defense {}, health {}",
            debug.stats, debug.speed, debug.speed_jump, debug.damage, debug.defense, debug.health,
        );

        player
    }

    pub fn spawn_first_tower_orc_lord() -> Character {
        let position: Position = Position::new(150.0, 250.0);

        spawn_orc_lord(position)
    }

    pub fn spawn_first_tower() -> Character {
        let position: Position = Position::new(150.0, 250.0);

        spawn_orc(position)
    }

    pub fn spawn_second_tower() -> Character {
        let position: Position = Position::new(1250.0, 250.0);

        spawn_orc(position)
    }

    pub fn spawn_third_tower() -> Character {
        let position: Position = Position::new(1650.0, 250.0);

        spawn_orc(position)
    }

    fn spawn_orc(position: Position) -> Character {
        let size: Size = Size::new(210.0, 210.0);

        let vitality: f32 = 1.0;
        let strength: f32 = 2.0;
        let agility: f32 = 1.0;
        let resistance: f32 = 1.0;
        let stats: Stats = Stats::new(vitality, strength, agility, resistance);

        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let move_animation: Animation = create_orc_move_animation();
        let attack_animation: Animation = create_orc_attack_animation();
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
            attack_animation,
            jump_animation,
            body_collision,
            foot_collision,
            weapon_collision_template,
        )
    }

    fn spawn_orc_lord(position: Position) -> Character {
        let size: Size = Size::new(210.0, 210.0);

        let vitality: f32 = 20.0;
        let strength: f32 = 4.0;
        let agility: f32 = 0.0;
        let resistance: f32 = 1.0;
        let stats: Stats = Stats::new(vitality, strength, agility, resistance);

        let character_type: CharacterTypes = CharacterTypes::Enemy;
        let move_animation: Animation = create_orc_move_animation();
        let attack_animation: Animation = create_orc_attack_animation();
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
            attack_animation,
            jump_animation,
            body_collision,
            foot_collision,
            weapon_collision_template,
        )
    }
}
