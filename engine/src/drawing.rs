use glm::*;
use mesh::model::Model;
use shader::ShaderProgram;

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
}
