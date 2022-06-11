use sdl2::image::LoadTexture;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct TextureManager<'a> {
    loader: Rc<TextureCreator<WindowContext>>,
    cache: HashMap<&'a str, Rc<Texture>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(loader: Rc<TextureCreator<WindowContext>>) -> Self {
        Self {
            loader,
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, texture_path: &'a str) -> Result<Rc<Texture>, String> {
        match self.cache.entry(texture_path) {
            Entry::Occupied(entry) => Ok(Rc::clone(entry.get())),
            Entry::Vacant(entry) => {
                let texture = Rc::new(self.loader.load_texture(texture_path)?);
                entry.insert(Rc::clone(&texture));
                Ok(texture)
            }
        }
    }
}
