use crate::{character::character::Character, GameState, MainState, TARGET_FPS};
use ggez::Context;

pub fn execute(ctx: &mut Context, main_state: &mut MainState) {
    while ctx.time.check_update_time(TARGET_FPS) {}

    main_state.level_up.process(ctx, &mut main_state.player);

    if main_state.level_up.confirmed {
    // if true {
        main_state.characters.clear();
        main_state.combat.turn = 0;
        main_state.level_up.confirmed = false;
        let cloned_player_character: Character = main_state.player.character.clone();
        main_state.characters.push(cloned_player_character);
        main_state.current_state = GameState::InCombat;
    }
}
