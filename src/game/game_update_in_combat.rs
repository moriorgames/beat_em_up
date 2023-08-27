use crate::{
    character::{
        character::Character, character_builder::character_builder, character_types::CharacterTypes,
    },
    combat::action::Action,
    enemy::enemy_behaviour::enemy_behavior::update_enemy_behaviour,
    MainState, TARGET_FPS, GameState,
};
use ggez::Context;

pub fn execute(ctx: &mut Context, main_state: &mut MainState) {
    while ctx.time.check_update_time(TARGET_FPS) {
        if let Some(player) = main_state.characters.first() {
            match player.character_type {
                CharacterTypes::Player => {
                    let player: Character = player.clone();
                    let player_actions: Vec<Action> = main_state.player_controls.handle_input(
                        ctx,
                        &player,
                        main_state.combat.turn,
                    );
                    for action in player_actions {
                        main_state.combat.add_action(action)
                    }

                    let enemy_actions: Vec<Action> = update_enemy_behaviour(
                        main_state.characters.clone(),
                        &player,
                        main_state.combat.turn,
                    );
                    for action in enemy_actions {
                        main_state.combat.add_action(action);
                    }
                }
                _ => (),
            }
        }

        main_state
            .combat
            .process(&mut main_state.characters, &main_state.world);

        main_state
            .characters
            .retain(|character| character.current_health > 0.0);

        if main_state.combat.turn > 2500 && main_state.characters.len() <= 1 {
            main_state.characters.clear();
            main_state.level_up.turn = 0;
            main_state.player.experience += 2;
            main_state.current_state = GameState::LevelUp;
        }

        if main_state.combat.turn == 2000 {
            main_state
                .characters
                .push(character_builder::spawn_first_tower_orc_lord());
        } else if main_state.combat.turn > 2000 {
        } else {
            if main_state.characters.len() < 7 {
                if main_state.combat.turn % 1033 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_second_tower());
                } else if main_state.combat.turn % 617 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_third_tower());
                } else if main_state.combat.turn % 293 == 0 {
                    main_state
                        .characters
                        .push(character_builder::spawn_first_tower());
                }
            }
        }
    }
}
