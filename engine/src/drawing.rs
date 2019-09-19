use gl::types::*;
use glm::*;
use mesh::model::Model;
use shader::ShaderProgram;
use utils::gl::{texture::*, *};

pub type Color4 = Vec4;
pub type Color3 = Vec3;

pub struct GameObject {
    pub name: String,
    pub position: Vec3,
    pub model: Model,
}

impl GameObject {
    pub fn new(name: &str, model: Model, pos: Vec3) -> GameObject {
        GameObject {
            name: name.to_string(),
            model: model,
            position: pos,
        }
    }

    pub fn get_draw_call<'a>(&'a mut self, program: &'a mut ShaderProgram) -> DrawCall<'a> {
        DrawCall::new(
            &mut self.model.vbo,
            &mut self.model.ebo,
            &mut self.model.textures,
            program,
            self.model.num_indices,
            self.model.index_type,
        )
    }
}

pub struct DrawCall<'a> {
    vao: VertexArray,
    vbo: &'a mut Buffer,
    ebo: &'a mut Buffer,
    textures: &'a mut [Texture],
    vertex_spec: VertexSpec,

    num_indices: i32,
    index_type: GLenum,

    program: &'a mut ShaderProgram,
}

impl<'a> DrawCall<'a> {
    pub fn new(
        vbo: &'a mut Buffer,
        ebo: &'a mut Buffer,
        textures: &'a mut [Texture],
        program: &'a mut ShaderProgram,
        num_indices: i32,
        index_type: GLenum,
    ) -> DrawCall<'a> {
        DrawCall {
            vao: VertexArray::new(),
            ebo: ebo,
            vbo: vbo,
            textures: textures,
            vertex_spec: VertexSpec::new(vec![
                VertexAttribute::new(0, gl::FLOAT, 3, false),
                VertexAttribute::new(1, gl::FLOAT, 3, false),
                VertexAttribute::new(2, gl::FLOAT, 3, false),
            ]),

            num_indices: num_indices,
            index_type: index_type,

            program: program,
        }
    }

    pub fn bind(&'a mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        self.vertex_spec.enable();
        self.program.use_program();

        let mut i = 0;
        for t in self.textures.iter() {
            t.bind_to_texture_unit(i);
            i += 1;
        }
    }

    pub fn perform(&mut self) {
        enable(Capability::CullFace);
        enable(Capability::DepthTest);
        draw_triangles(self.num_indices, self.index_type);
    }

    pub fn set_bool(&self, name: &str, val: bool) {
        self.program.set_bool(name, val);
    }

    pub fn set_int(&self, name: &str, val: i32) {
        self.program.set_int(name, val);
    }

    pub fn set_float(&self, name: &str, val: f32) {
        self.program.set_float(name, val);
    }

    pub fn set_float2(&self, name: &str, val: (f32, f32)) {
        self.program.set_float2(name, val);
    }

    pub fn set_float3(&self, name: &str, val: (f32, f32, f32)) {
        self.program.set_float3(name, val);
    }

    pub fn set_float4(&self, name: &str, val: (f32, f32, f32, f32)) {
        self.program.set_float4(name, val);
    }

    pub fn set_mat3(&self, name: &str, val: &Mat3) {
        self.program.set_mat3(name, val);
    }

    pub fn set_mat4(&self, name: &str, val: &Mat4) {
        self.program.set_mat4(name, val);
    }
}
