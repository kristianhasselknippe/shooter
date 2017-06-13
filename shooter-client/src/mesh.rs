use super::gl;
use super::gl::types::*;
use std::mem;
use std::ptr;

pub struct Mesh {
    vbo: GLuint,
    ebo: GLuint,

    vertices: Vec<GLfloat>,
    indices: Vec<GLuint>,
}

impl Mesh {
    pub fn new(vertices: Vec<GLfloat>, indices: Vec<GLuint>) -> Mesh {
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::GenBuffers(1, &mut ebo);

            //glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as isize, mem::transmute(vertices.first().unwrap()), gl::STATIC_DRAW);
        }

        Mesh {
            vbo: vbo,
            ebo: ebo,

            vertices: vertices,
            indices: indices,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&self.vertices[0]), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0 ,3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
                           mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);
        }
    }
}
