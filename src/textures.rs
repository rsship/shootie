use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use std::path::Path;

pub struct TextureLoader<'a> {
    pub textures: Vec<Texture<'a>>, 
    pub texture_creator: &'a TextureCreator<WindowContext>,
}


impl<'a> TextureLoader<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            textures: Vec::<Texture<'a>>::new(),
            texture_creator,
        }
    }

    pub fn add_texture<P: AsRef<Path>>(&mut self, file_path: P) {
        match self.texture_creator.load_texture(file_path) {
            Ok(texture) => {
                self.textures.push(texture);
            }, 
            Err(err) => {
                eprintln!("Error: file {} not loaded", err);
            }
        }
    }


}
