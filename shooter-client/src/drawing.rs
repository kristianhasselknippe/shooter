use gl;
use gl::types::*;
use utils::gl::*;
use shader::ShaderProgram;
use na::Matrix4;
use camera::Camera;

pub struct Color(f32, f32, f32, f32);

pub struct DrawContext {
    vao: VertexArray,
    width: u32,
    height: u32,
    camera: Camera,
}

impl DrawContext {
    pub fn new(width: u32, height: u32, camera: Camera) -> DrawContext {
        println!("Creating draw context");
        DrawContext {
            camera: camera,
            width: width,
            height: height,
            vao: gen_vertex_array(),
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn bind(&mut self) {
        self.vao.bind();
    }

    pub fn unbind(&mut self) {
        self.vao.unbind();
    }
}
