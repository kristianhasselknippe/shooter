use gl;
use gl::types::*;

struct TextureBinding<'a> {
    handle: &'a Texture
}

impl<'a> Drop for TextureBinding<'a> {
    fn drop(&mut self) {
        self.handle.unbind();
    }
}

#[derive(Debug,Clone)]
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

            Texture {
                handle: handle
            }
        }
    }

    pub fn bind_to_texture_unit(&self, unit: u32) {
        unsafe {
            match unit {
                0 => { gl::ActiveTexture(gl::TEXTURE0); },
                1 => { gl::ActiveTexture(gl::TEXTURE1); },
                2 => { gl::ActiveTexture(gl::TEXTURE2); },
                3 => { gl::ActiveTexture(gl::TEXTURE3); },
                4 => { gl::ActiveTexture(gl::TEXTURE4); },
                5 => { gl::ActiveTexture(gl::TEXTURE5); },
                6 => { gl::ActiveTexture(gl::TEXTURE6); },
                7 => { gl::ActiveTexture(gl::TEXTURE7); },
                8 => { gl::ActiveTexture(gl::TEXTURE8); },
                9 => { gl::ActiveTexture(gl::TEXTURE9); },
                10 => { gl::ActiveTexture(gl::TEXTURE10); },
                11 => { gl::ActiveTexture(gl::TEXTURE11); },
                12 => { gl::ActiveTexture(gl::TEXTURE12); },
                13 => { gl::ActiveTexture(gl::TEXTURE13); },
                14 => { gl::ActiveTexture(gl::TEXTURE14); },
                15 => { gl::ActiveTexture(gl::TEXTURE15); },
                16 => { gl::ActiveTexture(gl::TEXTURE16); },
                17 => { gl::ActiveTexture(gl::TEXTURE17); },
                18 => { gl::ActiveTexture(gl::TEXTURE18); },
                19 => { gl::ActiveTexture(gl::TEXTURE19); },
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
        println!("Uploading image of len: {} and dim: ({},{})", width*height, width, height);

        let format = match bbp {
            4 => {
                gl::RGBA
            },
            _ => panic!("Unsupported bbp: {}", bbp)
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
        self.unbind();
    }
}

pub fn bind_texture_unit(unit: u32, handle: GLuint) {
    unsafe {
        match unit {
            0 => { gl::ActiveTexture(gl::TEXTURE0); },
            1 => { gl::ActiveTexture(gl::TEXTURE1); },
            2 => { gl::ActiveTexture(gl::TEXTURE2); },
            3 => { gl::ActiveTexture(gl::TEXTURE3); },
            4 => { gl::ActiveTexture(gl::TEXTURE4); },
            5 => { gl::ActiveTexture(gl::TEXTURE5); },
            6 => { gl::ActiveTexture(gl::TEXTURE6); },
            7 => { gl::ActiveTexture(gl::TEXTURE7); },
            8 => { gl::ActiveTexture(gl::TEXTURE8); },
            9 => { gl::ActiveTexture(gl::TEXTURE9); },
            10 => { gl::ActiveTexture(gl::TEXTURE10); },
            11 => { gl::ActiveTexture(gl::TEXTURE11); },
            12 => { gl::ActiveTexture(gl::TEXTURE12); },
            13 => { gl::ActiveTexture(gl::TEXTURE13); },
            14 => { gl::ActiveTexture(gl::TEXTURE14); },
            15 => { gl::ActiveTexture(gl::TEXTURE15); },
            16 => { gl::ActiveTexture(gl::TEXTURE16); },
            17 => { gl::ActiveTexture(gl::TEXTURE17); },
            18 => { gl::ActiveTexture(gl::TEXTURE18); },
            19 => { gl::ActiveTexture(gl::TEXTURE19); },
            _ => {
                panic!("Unsupported texture unit {}", unit);
            }
        }
        gl::BindTexture(gl::TEXTURE_2D, handle);
    }
}
