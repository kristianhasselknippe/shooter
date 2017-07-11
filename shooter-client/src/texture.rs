use std::path::Path;
use std::fs::File;
use super::image::png::PNGDecoder;
use super::image::{DecodingResult,ColorType,ImageDecoder};
use super::gl;
use super::gl::types::*;
use super::drawing::DrawContext;
use std::os::raw::c_void;
use std::ptr;

use super::na::{Point2,Vector2};

#[derive(Clone)]
pub struct TextureHandle(GLuint);

pub struct Texture {
    handle: TextureHandle,
}

fn create_texture(dim: (u32,u32), data: Vec<u8>, format: GLenum) -> Texture {
    let handle = unsafe {
        let mut handle: GLuint = 0;
        gl::GenTextures(1, &mut handle);
        gl::BindTexture(gl::TEXTURE_2D, handle);
        gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32,
                       dim.0 as i32, dim.1 as i32, 0, format as u32, gl::UNSIGNED_BYTE,
                       data.as_ptr() as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        handle
    };

    Texture {
        handle: TextureHandle(handle),
    }
}

impl Texture {
    pub fn from_png(path: &Path) -> Texture {
        let image_file = File::open(path).unwrap();
        let mut decoder = PNGDecoder::new(image_file);
        let dim = decoder.dimensions().unwrap();
        let image_data = decoder.read_image().unwrap();
        let color_type = decoder.colortype().unwrap();

        match (color_type,image_data) {
            (ColorType::RGB(bit_depth),DecodingResult::U8(data)) => {
                panic!("Unsupported bit depth");
            },
            (ColorType::RGBA(bit_depth),DecodingResult::U8(data)) => {
                create_texture(dim, data, gl::RGBA)
            },
            _ => panic!("Unsupported color type and data type in image"),
        }
    }

    pub fn from_data_u8(dim: (i32,i32), data: Vec<u8>) -> Texture {
        let mut texture: GLuint = 0;

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);

            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32,
                           dim.0, dim.1,
                           0, gl::RED as u32, gl::UNSIGNED_BYTE,
                           data.as_ptr() as *const GLvoid);
        }

        Texture {
            handle: TextureHandle(texture)
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.handle.0);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }
}

pub struct MemoryTexture {
    data: Vec<u8>,
}

impl MemoryTexture {
    pub fn new(data: &[u8]) -> MemoryTexture {
        MemoryTexture {
            data: data.to_vec()
        }
    }
}

pub struct TextureAtlas {
    memory_textures: Vec<MemoryTexture>,
}

pub struct TextureAtlasRef(u32);

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        TextureAtlas {
            memory_textures: Vec::new()
        }
    }

    pub fn add_texture(&mut self, mem_tex: MemoryTexture) -> TextureAtlasRef {
        self.memory_textures.push(mem_tex);
        TextureAtlasRef(self.memory_textures.len() as u32)
    }

    pub fn draw(&mut self) -> Texture {
        next up we need to pack and draw the texture atlas
    }
}


struct Framebuffer {
    handle: GLuint,
    width: u32,
    height: u32,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Framebuffer {
        let mut handle = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut handle);
        }
        Framebuffer {
            handle: handle,
            width: width,
            height: height,
        }
    }

    pub fn bind(&self, dc: &DrawContext) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);

            let mut color_buffer = 0;
            gl::GenTextures(1, &mut color_buffer);
            gl::BindTexture(gl::TEXTURE_2D, color_buffer);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, dc.width as i32, dc.height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_buffer, 0);

            gl::Viewport(0, 0, self.width as i32, self.height as i32);


            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer is complete");
            }
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}


impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, self.handle as *const u32);
        }
    }
}
