use gl;
use utils::gl::{
    *,
    texture::*
};
use shader::ShaderProgram;
use na::*;
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
}

pub struct Transform {
    pos: Vector3<f32>
}

impl Transform {
    pub fn from_pos(pos: Vector3<f32>) -> Transform {
        Transform {
            pos: pos
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let isom = Isometry3::new(self.pos, zero());
        isom.to_homogeneous()
    }
}

pub struct DrawCall {
    vao: VertexArray,
    program: Rc<ShaderProgram>,
    pub model: Model,
    pub transform: Transform,
    vertex_attributes: Vec<VertexAttribute>,
}

impl DrawCall {
    pub fn new<T: Fn(&DrawCall)>(
        program: Rc<ShaderProgram>,
        model: Model,
        vertex_attributes: Vec<VertexAttribute>,
        transform: Transform,
        setup: T) -> DrawCall {
        let mut vao = gen_vertex_array();
        //println!("Genrated vao: {:?}", vao.handle);
        let mut model = model;
        vao.bind();
        program.use_program();
        model.bind();
        enable_vertex_attribs(&vertex_attributes);

        let mut ret = DrawCall {
            vao: vao,
            program: program,
            model: model,
            transform: transform,
            vertex_attributes: vertex_attributes,
        };

        setup(&mut ret);
        ret.vao.unbind();
        ret
    }

    pub fn draw(&mut self) {
        draw_triangles(self.model.num_indices, self.model.index_type);
    }

    pub fn bind(&mut self) {
        self.vao.bind();
    }

    pub fn unbind(&mut self) {
        self.vao.unbind();
    }

    pub fn set_mat3(&self, name: &str, val: &Matrix3<f32>) {
        self.program.set_mat3(name, val);
    }

    pub fn set_mat4(&self, name: &str, val: &Matrix4<f32>) {
        self.program.set_mat4(name, val);
    }

    pub fn set_vec3(&self, name: &str, val: &Vector3<f32>) {
        self.program.set_float3(name, (val.x, val.y, val.z));
    }

    pub fn bind_texture(&self, name: &str, val: &Texture, unit: u32) {
        val.bind_to_texture_unit(unit);
        self.program.set_int(name, unit as i32);
    }
}
