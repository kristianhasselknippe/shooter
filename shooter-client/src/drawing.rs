use gl;
use utils::gl::{
    *,
    texture::*
};
use shader::ShaderProgram;
use na::*;
use mesh::model::Model;
use std::rc::Rc;

pub type Color4 = Vector4<f32>;
pub type Color3 = Vector3<f32>;

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

    pub fn clear(&mut self, color: Color4) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

pub struct DrawCall {
    pub name: String,
    vao: VertexArray,
    program: Rc<ShaderProgram>,
    pub model: Model,
    pub position: Vector3<f32>,
    vertex_attributes: Vec<VertexAttribute>,
}

impl DrawCall {
    pub fn new<T: Fn(&DrawCall)>(
        name: &str,
        program: Rc<ShaderProgram>,
        model: Model,
        vertex_attributes: Vec<VertexAttribute>,
        position: Vector3<f32>,
        setup: T) -> DrawCall {
        let mut vao = gen_vertex_array();
        //println!("Genrated vao: {:?}", vao.handle);
        let mut model = model;
        vao.bind();
        program.use_program();
        model.bind();
        enable_vertex_attribs(&vertex_attributes);

        let mut ret = DrawCall {
            name: name.to_string(),
            vao: vao,
            program: program,
            model: model,
            position: position,
            vertex_attributes: vertex_attributes,
        };

        setup(&mut ret);
        ret.vao.unbind();
        ret
    }

    pub fn draw(&mut self) {
        unsafe {
            enable(Capability::CullFace);
            enable(Capability::DepthTest);
        }
        draw_triangles(self.model.num_indices, self.model.index_type);
    }

    pub fn bind(&mut self) {
        self.vao.bind();
        self.program.use_program();
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
