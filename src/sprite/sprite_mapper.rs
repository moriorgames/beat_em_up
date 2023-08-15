use std::collections::HashMap;

pub struct SpriteLoader {
    map: HashMap<u16, (String, String)>,
}

impl SpriteLoader {
    pub fn new() -> Self {
        let mut map: HashMap<u16, (String, String)> = HashMap::new();
        map.insert(
            1,
            (
                "barbarian_move_0".to_string(),
                "/barbarian_move_0.png".to_string(),
            ),
        );
        map.insert(
            2,
            (
                "barbarian_move_1".to_string(),
                "/barbarian_move_1.png".to_string(),
            ),
        );

        SpriteLoader { map }
    }

    pub fn get_sprite_values(&self) -> Vec<&(String, String)> {
        self.map.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sprite_map_returns_correct_textures() {
        let sut: SpriteLoader = SpriteLoader::new();
        let sprites: Vec<&(String, String)> = sut.get_sprite_values();
        assert_eq!(sprites.len(), 2);
        assert!(sprites.contains(&&("barbarian_move_0".to_string(), "/barbarian_move_0.png".to_string())));
        assert!(sprites.contains(&&("barbarian_move_1".to_string(), "/barbarian_move_1.png".to_string())));
    }

    #[test]
    fn get_sprite_map_does_not_contain_invalid_key() {
        let sut: SpriteLoader = SpriteLoader::new();
        let sprites: Vec<&(String, String)> = sut.get_sprite_values();
        assert_eq!(sprites.len(), 2);
        assert!(!sprites.contains(&&("invalid_key".to_string(), "/invalid_path.png".to_string())));
    }
}
