use crate::player::{level_up_controls::{LevelUpControls, LevelUpIntention}, player::Player};
use ggez::Context;

pub struct LevelUp {
    pub turn: u128,
    pub level_up_controls: LevelUpControls,
    selected_stat_index: usize,
    stat_list: Vec<String>,
}

impl LevelUp {
    pub fn new() -> Self {
        let level_up_controls: LevelUpControls = LevelUpControls::new();
        let stat_list: Vec<String> = vec!["vitality".to_string(), "strength".to_string(), "agility".to_string(), "resistance".to_string()];
        Self {
            turn: 0,
            level_up_controls,
            selected_stat_index: 0,
            stat_list,
        }
    }

    pub fn process(&mut self, ctx: &mut Context, player: &mut Player) {
        self.turn += 1;
        println!("Level up Turn: {:?}", self.turn);

        let intention: LevelUpIntention = self.level_up_controls.handle_input(ctx);
        
        if intention.move_up {
            if self.selected_stat_index > 0 {
                self.selected_stat_index -= 1;
            }
        }
        if intention.move_down {
            if self.selected_stat_index < self.stat_list.len() - 1 {
                self.selected_stat_index += 1;
            }
        }

        let selected_stat: &String = &self.stat_list[self.selected_stat_index].clone();
        
        if intention.move_right {
            self.increment_selected_stat(selected_stat, player);
        }
        if intention.move_left {
            self.decrement_selected_stat(selected_stat, player);
        }
    }

    fn increment_selected_stat(&mut self, selected_stat: &String, player: &mut Player) {
        if player.experience > 0 {
            match selected_stat.as_str() {
                "vitality" => {
                    player.character.stats.vitality += 1.0;
                    player.experience -= 1;
                },
                _ => {}
            }
        }
    }

    fn decrement_selected_stat(&mut self, selected_stat: &String, player: &mut Player) {
        if player.experience < player.max_experience {
            match selected_stat.as_str() {
                "vitality" => {
                    if player.character.stats.vitality > 0.0 {
                        player.character.stats.vitality -= 1.0;
                        player.experience += 1;
                    }
                },
                _ => {}
            }
        }
    }
}
