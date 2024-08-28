use std::path::Path;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use gl::types::*;

#[derive(Clone)]
pub struct Texture {
    texture_id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new() -> Self {
        Self {
            texture_id: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn load(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        file_name: &str,
    ) -> Result<(), String> {
        let sdl_texture = texture_creator.load_texture(Path::new(file_name))?;
        let query = sdl_texture.query();

        self.width = query.width;
        self.height = query.height;

        unsafe {
            gl::GenTextures(1, &mut self.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        }

        Ok(())
    }

    pub fn unload(&mut self) {
        if self.texture_id != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.texture_id);
            }
            self.texture_id = 0;
        }
    }

    pub fn set_active(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
