use super::sprite_mapper::SpriteMapper;
use ggez::graphics::Image;
use ggez::Context;
use std::collections::HashMap;

pub struct SpriteRepository {
    sprite_mapper: SpriteMapper,
    sprites: HashMap<String, Image>,
}

impl SpriteRepository {
    pub fn new(gfx: &mut Context) -> Self {
        let sprite_mapper: SpriteMapper = SpriteMapper::new();
        let sprite_tuples: Vec<&(String, String)> = sprite_mapper.get_sprite_map();
        let mut sprites: HashMap<String, Image> = HashMap::new();
        for (id, path) in sprite_tuples {
            let image: Image = Image::from_path(gfx, path).unwrap();
            sprites.insert(id.clone(), image);
        }

        Self {
            sprite_mapper,
            sprites,
        }
    }

    pub fn get_sprite(&self, id: &str) -> Option<&Image> {
        self.sprites.get(id)
    }
}
