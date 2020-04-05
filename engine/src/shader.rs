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
    pub fn create_vertex_shader(vs: &str) -> Shader {
        panic!();
    }

    pub fn create_fragment_shader(fs: &str) -> Shader {
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

    pub fn new(vs: &Shader, fs: &Shader) -> ShaderProgram {
        panic!();
    }

    pub fn from_fragments(vs: &str, fs: &str) -> ShaderProgram {
        panic!();
    }

    pub fn create_program(name: &str) -> ShaderProgram {
        panic!();
    }

    pub fn create_program_from_vert_frag(vert: &str, frag: &str) -> ShaderProgram {
        panic!();
    }

    pub fn set_bool(&self, name: &str, val: bool) {
    }

    pub fn set_int(&self, name: &str, val: i32) {
    }

    pub fn set_float(&self, name: &str, val: f32) {
    }

    pub fn set_float2(&self, name: &str, val: (f32, f32)) {
    }

    pub fn set_float3(&self, name: &str, val: (f32, f32, f32)) {
    }

    pub fn set_float4(&self, name: &str, val: (f32, f32, f32, f32)) {
    }

    pub fn set_mat3(&self, name: &str, val: &Mat3) {
    }

    pub fn set_mat4(&self, name: &str, val: &Mat4) {
    }
}
