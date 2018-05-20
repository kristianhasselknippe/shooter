use gl;
use gl::types::*;
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

pub struct GameObject {
    pub name: String,
    pub position: Vector3<f32>,
    pub model: Model,
}

impl GameObject {
    pub fn new(name: &str, model: Model, pos: Vector3<f32>) -> GameObject {
        GameObject {
            name: name.to_string(),
            model: model,
            position: pos,
        }
    }
}
