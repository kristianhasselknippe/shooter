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

    pub fn create_rect(w: f32, h: f32) -> Mesh {
        let w = w;
        let h = h;
        let vertices: Vec<GLfloat> = vec![
            //positions            //tex coords
            -w, -h, 0.0,        0.0,  1.0,
             w, -h, 0.0,        1.0,  1.0,
             w,  h, 0.0,        1.0,  0.0,
            -w,  h, 0.0,        0.0,  0.0,
        ];

        let indices: Vec<GLuint> = vec![  // Note that we start from 0!
            0, 1, 3,   // First Triangle
            1, 2, 3    // Second Triangle
        ];

        Mesh::new(vertices, indices)
    }

    pub fn create_from_topleft_bottomright(topleft: (f32,f32), bottomright: (f32,f32)) -> Mesh {
        let vertices: Vec<GLfloat> = vec![
            //positions            //tex coords
                topleft.0,      topleft.1, 0.0,        0.0,  1.0,
            bottomright.0,      topleft.1, 0.0,        1.0,  1.0,
            bottomright.0,  bottomright.1, 0.0,        1.0,  0.0,
                topleft.0,  bottomright.1, 0.0,        0.0,  0.0,
        ];
        let indices: Vec<GLuint> = vec![  // Note that we start from 0!
            0, 1, 3,   // First Triangle
            1, 2, 3    // Second Triangle
        ];

        Mesh::new(vertices, indices)
    }

    pub fn create_quad() -> Mesh {
        Self::create_rect(1.0,1.0)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&self.vertices[0]), gl::STATIC_DRAW);


            let mut positions = VertexAttribute::new(0, GLDataType::Float, 3);
            positions.enable(0,5);

            let mut ux_coords = VertexAttribute::new(1, GLDataType::Float, 2);
            ux_coords.enable(3,5);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
                           mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

enum GLDataType {
    Float,
}

struct VertexAttribute {
    n_components: u32,
    data_type: GLDataType,
    location: u32,
}

impl VertexAttribute {
    pub fn new(location: u32, data_type: GLDataType, n_components: u32) -> VertexAttribute {
        VertexAttribute {
            n_components: n_components,
            data_type: data_type,
            location: location,
        }
    }

    pub fn enable(&mut self, offset: u32, stride: u32) {
        unsafe {
            let (t, size) = match &self.data_type {
                &GLDataType::Float => { (gl::FLOAT, mem::size_of::<GLfloat>() as i32) }
            };
            gl::VertexAttribPointer(self.location, self.n_components as i32, t, gl::FALSE,
                                    (stride as i32 * size) as i32,
                                    (offset as i32 * size) as *const c_void);
            gl::EnableVertexAttribArray(self.location);
        }
    }
}

pub struct Batch {
    vertices: Vec<GLfloat>,
    indices: Vec<GLuint>,
}

impl Batch {
    pub fn new() -> Batch {
        Batch {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn write_mesh(&mut self, mesh: &Mesh) {

    }

    pub fn draw(&self) {
    }
}
