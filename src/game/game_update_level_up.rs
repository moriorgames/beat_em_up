use crate::{character::character::Character, GameState, MainState};
use ggez::Context;

pub fn execute(ctx: &mut Context, main_state: &mut MainState) {
    main_state.level_up.process(ctx, &mut main_state.player);

    if main_state.level_up.turn > 200 && main_state.player.experience == 0 {
        main_state.combat.turn = 0;
        let cloned_player_character: Character = main_state.player.character.clone();
        main_state.characters.push(cloned_player_character);
        main_state.current_state = GameState::InCombat;
    }
}
