pub mod character_builder {
    use crate::{
        character::{
            character::Character,
            characters_builder::{
                barbarian_builder::build_barbarian, orc_builder::build_orc,
                orc_lord_builder::build_orc_lord,
            },
        },
        geometry::position::Position,
    };

    pub fn build_player() -> Character {
        let player: Character = build_barbarian();
        let debug: Character = player.clone();
        println!(
            "- player {:?} speed {}, speed_jump {}, damage {}, defense {}, health {}",
            debug.stats,
            debug.speed,
            debug.speed_jump,
            debug.fast_damage,
            debug.defense,
            debug.health,
        );

        player
    }

    pub fn spawn_first_tower_orc_lord() -> Character {
        let position: Position = Position::new(150.0, 250.0);

        build_orc_lord(position)
    }

    pub fn spawn_first_tower() -> Character {
        let position: Position = Position::new(150.0, 250.0);

        build_orc(position)
    }

    pub fn spawn_second_tower() -> Character {
        let position: Position = Position::new(1250.0, 250.0);

        build_orc(position)
    }

    pub fn spawn_third_tower() -> Character {
        let position: Position = Position::new(1650.0, 250.0);

        build_orc(position)
    }
}
