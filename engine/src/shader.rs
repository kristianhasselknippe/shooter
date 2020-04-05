use std::ffi::CString;
use std::path::Path;
use std::ptr;
use std::str;
use utils::file::{abolute_path_from_relative, read_file};

use glm::{Mat3, Mat4};

pub struct Shader<THandle = u32> {
    handle: THandle,
}

const SHADERS_PATH: &'static str = "../engine/src/shaders";

impl Shader {
    pub fn create_vertex_shader(_vs: &str) -> Shader {
        panic!();
    }

    pub fn create_fragment_shader(_fs: &str) -> Shader {
        panic!();
    }

    pub fn create_fragment_shader_from_path(path: &Path) -> Shader {
        let fs = read_file(path).unwrap();
        Shader::create_fragment_shader(&fs)
    }

    pub fn create_vertex_shader_from_path(path: &Path) -> Shader {
        let vs = read_file(path).unwrap();
        Shader::create_vertex_shader(&vs)
    }
}

pub struct ShaderProgram<THandle = u32> {
    pub handle: THandle,
}

impl ShaderProgram {
    pub fn use_program(&self) {
        panic!();
    }

    pub fn new(_vs: &Shader, _fs: &Shader) -> ShaderProgram {
        panic!();
    }

    pub fn from_fragments(_vs: &str, _fs: &str) -> ShaderProgram {
        panic!();
    }

    pub fn create_program(_name: &str) -> ShaderProgram {
        panic!();
    }

    pub fn create_program_from_vert_frag(_vert: &str, _frag: &str) -> ShaderProgram {
        panic!();
    }

    pub fn set_bool(&self, _name: &str, _val: bool) {
    }

    pub fn set_int(&self, _name: &str, _val: i32) {
    }

    pub fn set_float(&self, _name: &str, _val: f32) {
    }

    pub fn set_float2(&self, _name: &str, _val: (f32, f32)) {
    }

    pub fn set_float3(&self, _name: &str, _val: (f32, f32, f32)) {
    }

    pub fn set_float4(&self, _name: &str, _val: (f32, f32, f32, f32)) {
    }

    pub fn set_mat3(&self, _name: &str, _val: &Mat3) {
    }

    pub fn set_mat4(&self, _name: &str, _val: &Mat4) {
    }
}
