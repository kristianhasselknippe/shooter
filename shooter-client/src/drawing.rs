use gl;
use utils::gl::*;
use shader::ShaderProgram;
use na::Matrix4;
use mesh::model::Model;
use std::rc::Rc;

pub struct Color(f32, f32, f32, f32);

pub struct DrawContext {
    width: u32,
    height: u32,
}

impl DrawContext {
    pub fn new(width: u32, height: u32) -> DrawContext {
        println!("Creating draw context");
        DrawContext {
            width: width,
            height: height,
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self, draw_call: &mut DrawCall) {
        draw_call.draw();
    }
}

pub struct DrawCall {
    vao: VertexArray,
    program: Rc<ShaderProgram>,
    model: Model,
    vertex_attributes: Vec<VertexAttribute>,
}

impl DrawCall {
    pub fn new(program: Rc<ShaderProgram>, model: Model, vertex_attributes: Vec<VertexAttribute>) -> DrawCall {
        let mut vao = gen_vertex_array();
        let mut model = model;
        vao.bind();
        program.use_program();
        model.bind();
        enable_vertex_attribs(&vertex_attributes);
        vao.unbind();
                
        DrawCall {
            vao: vao,
            program: program,
            model: model,
            vertex_attributes: vertex_attributes,
        }
    }

    pub fn draw(&mut self) {
        self.bind();
        draw_triangles(self.model.num_indices, self.model.index_type);
        self.unbind();
    }
    
    pub fn bind(&mut self) {
        self.vao.bind();
    }

    pub fn unbind(&mut self) {
        self.vao.unbind();
    }

    pub fn set_mat4(&self, name: &str, val: &Matrix4<f32>) {
        self.program.set_mat4(name, val);
    }   
}
