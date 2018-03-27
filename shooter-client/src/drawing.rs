use shader::ShaderProgram;
use na::{Matrix4};


pub struct Color(f32,f32,f32,f32);

pub struct DrawContext {
    vao: VertexArray,
    width: u32,
    height: u32,
}

impl DrawContext {
    pub fn new(width: u32, height: u32, camera_matrix: Matrix4<f32>) -> DrawContext {
        DrawContext {
            camera_matrix: camera_matrix,
            width: width,
            height: height,
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.0,color.1,color.2,color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self) {
        self.vao.bind();
        
        self.vao.unbind();
    }

}
