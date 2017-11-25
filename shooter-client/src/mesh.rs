use super::drawing::*;
use super::gl;
use super::gl::types::*;
use std::mem;
use std::ptr;
use super::na::Vector2;

pub struct Mesh {
    vbo: GLuint,
    ebo: GLuint,

    pub n_elements: i32,

    pub vertices: Vec<GLfloat>,
    pub indices: Vec<GLuint>,
}

impl Mesh {
    pub fn new(vertices: Vec<GLfloat>, indices: Vec<GLuint>) -> Mesh {
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            /*gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);*/


        }

        Mesh {
            vbo: vbo,
            ebo: ebo,

            n_elements: indices.len() as i32,

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

    pub fn create_from_pos_size(pos: Vector2<f64>, size: Vector2<f64>) -> Mesh {
        let top_left = (pos.x as f32, pos.y as f32);
        let bottom_right = (pos.x as f32 + size.x as f32, pos.y as f32 + size.y as f32);
        Mesh::create_from_topleft_bottomright(top_left, bottom_right)
    }

    pub fn create_quad() -> Mesh {
        Self::create_rect(1.0,1.0)
    }
}

impl Drawable for Mesh {
    fn draw(&self, dc: &DrawContext) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            let mut positions = VertexAttribute::new(0, GLDataType::Float, 3);
            positions.enable(0,5);

            let mut ux_coords = VertexAttribute::new(1, GLDataType::Float, 2);
            ux_coords.enable(3,5);

            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * self.vertices.len()) as isize,
                           mem::transmute(self.vertices.first().unwrap()), gl::STATIC_DRAW);

            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
                           mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);

            gl::DrawElements(gl::TRIANGLES, self.n_elements as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
