use super::gl;
use super::gl::types::*;
use std::mem;
use std::ptr;
use std::os::raw::c_void;

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
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as isize,
                           mem::transmute(vertices.first().unwrap()), gl::STATIC_DRAW);
        }

        Mesh {
            vbo: vbo,
            ebo: ebo,

            vertices: vertices,
            indices: indices,
        }
    }

    pub fn create_quad() -> Mesh {
        let vertices: Vec<GLfloat> = vec![
            //positions            //tex coords
            -1.0, -1.0, 0.0,        0.0,  0.0,
             1.0, -1.0, 0.0,        1.0,  0.0,
             1.0,  1.0, 0.0,        1.0,  1.0,
            -1.0,  1.0, 0.0,        0.0,  1.0,
        ];

        let indices: Vec<GLuint> = vec![  // Note that we start from 0!
            0, 1, 3,   // First Triangle
            1, 2, 3    // Second Triangle
        ];

        Mesh::new(vertices, indices)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&self.vertices[0]), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0 ,3, gl::FLOAT, gl::FALSE,
                                    (5 * mem::size_of::<GLfloat>()) as i32,
                                    0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1 ,2, gl::FLOAT, gl::FALSE,
                                    (5 * mem::size_of::<GLfloat>()) as i32,
                                    (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
                           mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);
        }
    }
}
