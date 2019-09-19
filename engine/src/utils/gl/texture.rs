#![allow(dead_code)]
use gl::types::*;

struct TextureBinding<'a> {
    handle: &'a Texture,
}

impl<'a> Drop for TextureBinding<'a> {
    fn drop(&mut self) {
        self.handle.unbind();
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub handle: GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        unsafe {
            let mut handle: GLuint = 0;
            gl::GenTextures(1, &mut handle);

            gl::BindTexture(gl::TEXTURE_2D, handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture { handle: handle }
        }
    }

    pub fn bind_to_texture_unit(&self, unit: u32) {
        unsafe {
            match unit {
                0 => {
                    gl::ActiveTexture(gl::TEXTURE0);
                }
                1 => {
                    gl::ActiveTexture(gl::TEXTURE1);
                }
                2 => {
                    gl::ActiveTexture(gl::TEXTURE2);
                }
                3 => {
                    gl::ActiveTexture(gl::TEXTURE3);
                }
                4 => {
                    gl::ActiveTexture(gl::TEXTURE4);
                }
                5 => {
                    gl::ActiveTexture(gl::TEXTURE5);
                }
                6 => {
                    gl::ActiveTexture(gl::TEXTURE6);
                }
                7 => {
                    gl::ActiveTexture(gl::TEXTURE7);
                }
                8 => {
                    gl::ActiveTexture(gl::TEXTURE8);
                }
                9 => {
                    gl::ActiveTexture(gl::TEXTURE9);
                }
                10 => {
                    gl::ActiveTexture(gl::TEXTURE10);
                }
                11 => {
                    gl::ActiveTexture(gl::TEXTURE11);
                }
                12 => {
                    gl::ActiveTexture(gl::TEXTURE12);
                }
                13 => {
                    gl::ActiveTexture(gl::TEXTURE13);
                }
                14 => {
                    gl::ActiveTexture(gl::TEXTURE14);
                }
                15 => {
                    gl::ActiveTexture(gl::TEXTURE15);
                }
                16 => {
                    gl::ActiveTexture(gl::TEXTURE16);
                }
                17 => {
                    gl::ActiveTexture(gl::TEXTURE17);
                }
                18 => {
                    gl::ActiveTexture(gl::TEXTURE18);
                }
                19 => {
                    gl::ActiveTexture(gl::TEXTURE19);
                }
                _ => {
                    panic!("Unsupported texture unit {}", unit);
                }
            }
            self.bind();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn upload(&mut self, pixels: *const u8, width: u32, height: u32, bbp: i32) {
        self.bind();
        println!(
            "Uploading image of len: {} and dim: ({},{})",
            width * height,
            width,
            height
        );

        let format = match bbp {
            4 => gl::RGBA,
            _ => panic!("Unsupported bbp: {}", bbp),
        };

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as _,
                width as GLsizei,
                height as GLsizei,
                0,
                format as _,
                gl::UNSIGNED_BYTE,
                pixels as _,
            );
        }
        println!("Done uploading texture");
    }
}

pub fn set_active_unit(unit: u32) {
    unsafe {
        match unit {
            0 => {
                gl::ActiveTexture(gl::TEXTURE0);
            }
            1 => {
                gl::ActiveTexture(gl::TEXTURE1);
            }
            2 => {
                gl::ActiveTexture(gl::TEXTURE2);
            }
            3 => {
                gl::ActiveTexture(gl::TEXTURE3);
            }
            4 => {
                gl::ActiveTexture(gl::TEXTURE4);
            }
            5 => {
                gl::ActiveTexture(gl::TEXTURE5);
            }
            6 => {
                gl::ActiveTexture(gl::TEXTURE6);
            }
            7 => {
                gl::ActiveTexture(gl::TEXTURE7);
            }
            8 => {
                gl::ActiveTexture(gl::TEXTURE8);
            }
            9 => {
                gl::ActiveTexture(gl::TEXTURE9);
            }
            10 => {
                gl::ActiveTexture(gl::TEXTURE10);
            }
            11 => {
                gl::ActiveTexture(gl::TEXTURE11);
            }
            12 => {
                gl::ActiveTexture(gl::TEXTURE12);
            }
            13 => {
                gl::ActiveTexture(gl::TEXTURE13);
            }
            14 => {
                gl::ActiveTexture(gl::TEXTURE14);
            }
            15 => {
                gl::ActiveTexture(gl::TEXTURE15);
            }
            16 => {
                gl::ActiveTexture(gl::TEXTURE16);
            }
            17 => {
                gl::ActiveTexture(gl::TEXTURE17);
            }
            18 => {
                gl::ActiveTexture(gl::TEXTURE18);
            }
            19 => {
                gl::ActiveTexture(gl::TEXTURE19);
            }
            _ => {
                panic!("Unsupported texture unit {}", unit);
            }
        }
    }
}

pub fn bind_texture_unit(unit: u32, handle: GLuint) {
    unsafe {
        set_active_unit(unit);
        gl::BindTexture(gl::TEXTURE_2D, handle);
    }
}

pub enum TextureInfoType {
    Width,
    Height,
}

pub fn get_texture_info_i(info_type: TextureInfoType) -> i32 {
    unsafe {
        let mut out: GLint = 0;
        match info_type {
            TextureInfoType::Width => gl::GetTexLevelParameteriv(
                gl::TEXTURE_2D,
                0,
                gl::TEXTURE_WIDTH,
                &mut out as *mut GLint,
            ),
            TextureInfoType::Height => gl::GetTexLevelParameteriv(
                gl::TEXTURE_2D,
                0,
                gl::TEXTURE_HEIGHT,
                &mut out as *mut GLint,
            ),
        }
        out
    }
}

pub fn get_texture_dim(unit: u32) -> (i32, i32) {
    set_active_unit(unit);
    let width = get_texture_info_i(TextureInfoType::Width);
    let height = get_texture_info_i(TextureInfoType::Height);
    (width, height)
}

pub fn read_pixels_from_texture2d(unit: u32) -> (Vec<u8>, (i32, i32)) {
    unsafe {
        let dim = get_texture_dim(unit);
        println!("Texture dim: {:?}", dim);
        let buffer_len = (dim.0 * dim.1 * 4) as usize;
        let mut out: Vec<u8> = vec![0; buffer_len];
        println!("Getting text img");
        gl::GetTexImage(
            gl::TEXTURE_2D,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            out.as_mut_ptr() as _,
        );
        println!("Done getting tex img");
        (out, dim)
    }
}
