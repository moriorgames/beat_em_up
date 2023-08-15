use std::collections::HashMap;

pub struct SpriteMapper {
    map: HashMap<u16, (String, String)>,
}

impl SpriteMapper {
    pub fn new() -> Self {
        let mut map: HashMap<u16, (String, String)> = HashMap::new();
        map.insert(
            1,
            (
                "barbarian_move_1".to_string(),
                "/barbarian_move_1.png".to_string(),
            ),
        );
        map.insert(
            2,
            (
                "barbarian_move_2".to_string(),
                "/barbarian_move_2.png".to_string(),
            ),
        );

        SpriteMapper { map }
    }

    pub fn get_sprite_map(&self) -> Vec<&(String, String)> {
        self.map.values().collect()
    }

    pub fn map(&self, key: u16) -> Option<&String> {
        self.map.get(&key).map(|value| &value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_returns_correct_texture() {
        let mapper: SpriteMapper = SpriteMapper::new();
        assert_eq!(mapper.map(1), Some(&"barbarian_move_1".to_string()));
        assert_eq!(mapper.map(2), Some(&"barbarian_move_2".to_string()));
    }

    #[test]
    fn map_returns_none_for_invalid_key() {
        let mapper: SpriteMapper = SpriteMapper::new();
        assert_eq!(mapper.map(99), None);
    }
}
