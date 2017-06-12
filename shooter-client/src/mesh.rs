use super::gl;
use super::gl::types::*;

pub struct Mesh {
    vertices: Vec<GLfloat>,
    indices: Vec<GLuint>,
}

impl Mesh {
    pub fn new(vertices: Vec<GLfloat>, indices: Vec<GLuint>) -> Mesh {
        Mesh {
            vertices: vertices,
            indices: indices,
        }
    }

    pub fn bind(&mut self) {

    }
}
