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

struct Image {
    data: Vec<u8>,
    dim: (u32,u32)
}

impl Image {
    pub fn from_png(path: &Path) -> Image {
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
                Image {
                    data: data,
                    dim: dim,
                }
            },
            _ => panic!("Unsupported color type and data type in image"),
        }

    }
}

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
        let img = Image::from_png(path);
        create_texture(img.dim, img.data, gl::RGBA)
    }

    pub fn from_data_u8(dim: (i32,i32), data: &Vec<u8>) -> Texture {
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

    pub fn bind(&self, unit: TextureUnit) {
        unsafe {
            gl::ActiveTexture(unit.as_gl_type());
            gl::BindTexture(gl::TEXTURE_2D, self.handle.0);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }
}

pub enum TextureUnit {
    Unit0,
    Unit1,
    Unit2,
    Unit3,
    Unit4,
    Unit5,
    Unit6,
    Unit7,
    Unit8,
    Unit9,
    Unit10,
    Unit11,
    Unit12,
    Unit13,
    Unit14,
    Unit15,
    Unit16,
    Unit17,
    Unit18,
    Unit19,
    Unit20,
    Unit21,
    Unit22,
    Unit23,
    Unit24,
    Unit25,
    Unit26,
    Unit27,
    Unit28,
    Unit29,
    Unit30,
    Unit31,
}

impl TextureUnit {
    pub fn as_gl_type(self) -> gl::types::GLenum {
        match self {
            TextureUnit::Unit0 =>  gl::TEXTURE0,
            TextureUnit::Unit1 =>  gl::TEXTURE1,
            TextureUnit::Unit2 =>  gl::TEXTURE2,
            TextureUnit::Unit3 =>  gl::TEXTURE3,
            TextureUnit::Unit4 =>  gl::TEXTURE4,
            TextureUnit::Unit5 =>  gl::TEXTURE5,
            TextureUnit::Unit6 =>  gl::TEXTURE6,
            TextureUnit::Unit7 =>  gl::TEXTURE7,
            TextureUnit::Unit8 =>  gl::TEXTURE8,
            TextureUnit::Unit9 =>  gl::TEXTURE9,
            TextureUnit::Unit10 => gl::TEXTURE10,
            TextureUnit::Unit11 => gl::TEXTURE11,
            TextureUnit::Unit12 => gl::TEXTURE12,
            TextureUnit::Unit13 => gl::TEXTURE13,
            TextureUnit::Unit14 => gl::TEXTURE14,
            TextureUnit::Unit15 => gl::TEXTURE15,
            TextureUnit::Unit16 => gl::TEXTURE16,
            TextureUnit::Unit17 => gl::TEXTURE17,
            TextureUnit::Unit18 => gl::TEXTURE18,
            TextureUnit::Unit19 => gl::TEXTURE19,
            TextureUnit::Unit20 => gl::TEXTURE20,
            TextureUnit::Unit21 => gl::TEXTURE21,
            TextureUnit::Unit22 => gl::TEXTURE22,
            TextureUnit::Unit23 => gl::TEXTURE23,
            TextureUnit::Unit24 => gl::TEXTURE24,
            TextureUnit::Unit25 => gl::TEXTURE25,
            TextureUnit::Unit26 => gl::TEXTURE26,
            TextureUnit::Unit27 => gl::TEXTURE27,
            TextureUnit::Unit28 => gl::TEXTURE28,
            TextureUnit::Unit29 => gl::TEXTURE29,
            TextureUnit::Unit30 => gl::TEXTURE30,
            TextureUnit::Unit31 => gl::TEXTURE31,
        }
    }
}

pub struct MemoryTexture {
    data: Vec<u8>,
    size: (u32,u32),
}

impl MemoryTexture {
    pub fn new(data: &[u8], width: u32, height: u32) -> MemoryTexture {
        MemoryTexture {
            data: data.to_vec(),
            size: (width, height),
        }
    }

    pub fn from_png(path: &Path) -> MemoryTexture {
        let img = Image::from_png(path);
        MemoryTexture::new(&img.data, img.dim.0, img.dim.1)
    }

    pub fn draw(&self, pos: (u32, u32)) {
        let texture = Texture::from_data_u8((pos.0 as i32, pos.1 as i32), &self.data);
        texture.bind(TextureUnit::Unit0);
    }
}

pub struct TextureAtlas {
    memory_textures: Vec<MemoryTexture>,
    size: (u32,u32),
}

pub struct TextureAtlasRef(u32);

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        TextureAtlas {
            memory_textures: Vec::new(),
            size: (0,0),
        }
    }

    pub fn add_texture(&mut self, mem_tex: MemoryTexture) -> TextureAtlasRef {
        self.memory_textures.push(mem_tex);
        TextureAtlasRef(self.memory_textures.len() as u32)
    }

    pub fn pack_and_draw(&mut self) -> Result<Texture,()> {
        for tex in &self.memory_textures {

        }

        Err(())
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
