use std::path::Path;
use std::fs::File;
use super::image::png::PNGDecoder;
use super::image::{DecodingResult,ColorType,ImageDecoder};
use super::gl;
use super::gl::types::*;
use std::os::raw::c_void;


pub struct Texture {
    handle: GLuint,
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
        handle: handle,
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

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Texture {
            handle: texture
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }
}
