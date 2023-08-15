use super::sprite_loader::SpriteLoader;
use ggez::graphics::Image;
use ggez::Context;
use std::collections::HashMap;

pub struct SpriteRepository {
    sprites: HashMap<String, Image>,
}

impl SpriteRepository {
    pub fn new(gfx: &mut Context) -> Self {
        let sprite_mapper: SpriteLoader = SpriteLoader::new();
        let sprite_tuples: &Vec<(String, String)> = sprite_mapper.get_sprite_values();
        let mut sprites: HashMap<String, Image> = HashMap::new();
        for (id, path) in sprite_tuples {
            let image: Image = Image::from_path(gfx, path).unwrap();
            sprites.insert(id.clone(), image);
        }

        Self {
            sprites,
        }
    }

    pub fn get_sprite(&self, id: &str) -> Option<&Image> {
        self.sprites.get(id)
    }
}
