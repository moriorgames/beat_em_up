use crate::player::{
    level_up_controls::{LevelUpControls, LevelUpIntention},
    player::Player,
};
use ggez::Context;

pub struct LevelUp {
    pub turn: u128,
    pub level_up_controls: LevelUpControls,
    pub confirmed: bool,
    selected_stat_index: usize,
    stat_list: Vec<String>,
}

impl LevelUp {
    pub fn new() -> Self {
        let level_up_controls: LevelUpControls = LevelUpControls::new();
        let stat_list: Vec<String> = vec![
            "vitality".to_string(),
            "strength".to_string(),
            "agility".to_string(),
            "resistance".to_string(),
            "confirm".to_string(),
        ];
        Self {
            turn: 0,
            level_up_controls,
            confirmed: false,
            selected_stat_index: 0,
            stat_list,
        }
    }

    pub fn process(&mut self, ctx: &mut Context, player: &mut Player) {
        self.turn += 1;
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

        if selected_stat == "confirm" && intention.confirm {
            self.confirm_stats(player);
        } else if intention.move_right {
            self.increment_selected_stat(selected_stat, player);
        } else if intention.move_left {
            self.decrement_selected_stat(selected_stat, player);
        }
    }

    pub fn get_selected_stat(&mut self) -> &String {
        &self.stat_list[self.selected_stat_index]
    }

    fn increment_selected_stat(&mut self, selected_stat: &String, player: &mut Player) {
        self.update_stat(player, selected_stat, 1.0);
    }

    fn decrement_selected_stat(&mut self, selected_stat: &String, player: &mut Player) {
        self.update_stat(player, selected_stat, -1.0);
    }

    fn update_stat(&mut self, player: &mut Player, stat: &str, value: f32) {
        if value > 0.0 && player.experience == 0 {
            return;
        }
        if value < 0.0 && player.experience >= player.max_experience {
            return;
        }

        match stat {
            "vitality" => {
                if player.character.stats.vitality + value >= 0.0 {
                    player.character.stats.vitality += value;
                    player.experience = (player.experience as i32 - value as i32) as u32;
                    player.character.rebuild_stats();

                    return;
                }
            }
            "strength" => {
                if player.character.stats.strength + value >= 0.0 {
                    player.character.stats.strength += value;
                    player.experience = (player.experience as i32 - value as i32) as u32;
                    player.character.rebuild_stats();

                    return;
                }
            }
            "agility" => {
                if player.character.stats.agility + value >= 0.0 {
                    player.character.stats.agility += value;
                    player.experience = (player.experience as i32 - value as i32) as u32;
                    player.character.rebuild_stats();

                    return;
                }
            }
            "resistance" => {
                if player.character.stats.resistance + value >= 0.0 {
                    player.character.stats.resistance += value;
                    player.experience = (player.experience as i32 - value as i32) as u32;
                    player.character.rebuild_stats();

                    return;
                }
            }
            _ => {}
        }
    }

    fn confirm_stats(&mut self, player: &mut Player) {
        if player.experience == 0 {
            self.confirmed = true;
        }
    }
}
