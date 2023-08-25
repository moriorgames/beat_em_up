pub struct SpriteLoader {
    sprites: Vec<(String, String)>,
}

impl SpriteLoader {
    pub fn new() -> Self {
        let mut sprites: Vec<(String, String)> = Vec::new();
        sprites.push((
            "barbarian_move_0".to_string(),
            "/barbarian_move_0.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_1".to_string(),
            "/barbarian_move_1.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_2".to_string(),
            "/barbarian_move_2.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_3".to_string(),
            "/barbarian_move_3.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_4".to_string(),
            "/barbarian_move_4.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_5".to_string(),
            "/barbarian_move_5.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_6".to_string(),
            "/barbarian_move_6.png".to_string(),
        ));
        sprites.push((
            "barbarian_move_7".to_string(),
            "/barbarian_move_7.png".to_string(),
        ));
        sprites.push((
            "barbarian_attack_0".to_string(),
            "/barbarian_attack_0.png".to_string(),
        ));
        sprites.push((
            "barbarian_attack_1".to_string(),
            "/barbarian_attack_1.png".to_string(),
        ));
        sprites.push((
            "barbarian_attack_2".to_string(),
            "/barbarian_attack_2.png".to_string(),
        ));
        sprites.push((
            "barbarian_attack_3".to_string(),
            "/barbarian_attack_3.png".to_string(),
        ));
        sprites.push((
            "barbarian_attack_4".to_string(),
            "/barbarian_attack_4.png".to_string(),
        ));
        sprites.push((
            "barbarian_jump_0".to_string(),
            "/barbarian_jump_0.png".to_string(),
        ));
        sprites.push((
            "barbarian_jump_1".to_string(),
            "/barbarian_jump_1.png".to_string(),
        ));
        sprites.push((
            "barbarian_jump_2".to_string(),
            "/barbarian_jump_2.png".to_string(),
        ));
        sprites.push((
            "barbarian_level_up".to_string(),
            "/barbarian_level_up.png".to_string(),
        ));

        sprites.push(("orc_move_0".to_string(), "/orc_move_0.png".to_string()));
        sprites.push(("orc_move_1".to_string(), "/orc_move_1.png".to_string()));
        sprites.push(("orc_move_2".to_string(), "/orc_move_2.png".to_string()));
        sprites.push(("orc_move_3".to_string(), "/orc_move_3.png".to_string()));
        sprites.push(("orc_move_4".to_string(), "/orc_move_4.png".to_string()));
        sprites.push(("orc_move_5".to_string(), "/orc_move_5.png".to_string()));

        sprites.push(("orc_attack_0".to_string(), "/orc_attack_0.png".to_string()));
        sprites.push(("orc_attack_1".to_string(), "/orc_attack_1.png".to_string()));
        sprites.push(("orc_attack_2".to_string(), "/orc_attack_2.png".to_string()));
        sprites.push(("orc_attack_3".to_string(), "/orc_attack_3.png".to_string()));
        sprites.push(("orc_attack_4".to_string(), "/orc_attack_4.png".to_string()));

        sprites.push(("orc_jump_0".to_string(), "/orc_jump_0.png".to_string()));
        sprites.push(("orc_jump_1".to_string(), "/orc_jump_1.png".to_string()));
        sprites.push(("orc_jump_2".to_string(), "/orc_jump_2.png".to_string()));

        sprites.push(("world".to_string(), "/world.png".to_string()));

        sprites.push(("fire_0".to_string(), "/fire_0.png".to_string()));
        sprites.push(("fire_1".to_string(), "/fire_1.png".to_string()));
        sprites.push(("fire_2".to_string(), "/fire_2.png".to_string()));
        sprites.push(("fire_3".to_string(), "/fire_3.png".to_string()));
        sprites.push(("fire_4".to_string(), "/fire_4.png".to_string()));
        sprites.push(("fire_5".to_string(), "/fire_5.png".to_string()));
        sprites.push(("fire_6".to_string(), "/fire_6.png".to_string()));
        sprites.push(("fire_7".to_string(), "/fire_7.png".to_string()));

        sprites.push(("fire_mask".to_string(), "/fire_mask.png".to_string()));

        SpriteLoader { sprites }
    }

    pub fn get_sprite_values(&self) -> &Vec<(String, String)> {
        &self.sprites
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_loader_returns_correct_sprites() {
        let sut: SpriteLoader = SpriteLoader::new();
        let sprites: &Vec<(String, String)> = sut.get_sprite_values();

        assert!(sprites.contains(&&(
            "barbarian_move_0".to_string(),
            "/barbarian_move_0.png".to_string()
        )));
        assert!(sprites.contains(&&(
            "barbarian_move_1".to_string(),
            "/barbarian_move_1.png".to_string()
        )));
    }

    #[test]
    fn sprite_loader_does_not_contain_invalid_key() {
        let sut: SpriteLoader = SpriteLoader::new();
        let sprites: &Vec<(String, String)> = sut.get_sprite_values();

        assert!(!sprites.contains(&&("invalid_key".to_string(), "/invalid_path.png".to_string())));
    }
}
