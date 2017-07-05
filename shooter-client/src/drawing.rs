use super::gl;
use super::gl::types::*;
use std::ptr;

pub struct DrawContext {
    vao: GLuint,
    pub width: u32,
    pub height: u32,
}

impl DrawContext {
    pub fn new(width: u32, height: u32) -> DrawContext {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        };
        DrawContext {
            vao: vao,
            width: width,
            height: height,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0); //unbind vao
        }
    }

    pub fn clear(&self, color: (f32,f32,f32,f32)) {
        unsafe {
            gl::ClearColor(color.0,color.1,color.2,color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
