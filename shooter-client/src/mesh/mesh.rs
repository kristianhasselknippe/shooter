/*use utils::gl::*;
use na::Vector2;
use super::Vertex;

pub struct MemoryMesh {
    vertices: Vec<Vertex<f64>>,
}

pub struct GlMesh {
    vbo: GLuint,
    ebo: GLuint,
}

impl GlMesh {
    pub fn new(vertices: Vec<GLfloat>, indices: Vec<GLuint>) -> GlMesh {
        let vbo = gen_buffers(1);
        let ebo = gen_buffers(1);

       
        /*gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);*/
        

        GlMesh {
            vbo: vbo,
            ebo: ebo,
        }
    }

    pub fn create_rect(w: f32, h: f32) -> GlMesh {
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

        GlMesh::new(vertices, indices)
    }

    pub fn create_from_topleft_bottomright(topleft: (f32,f32), bottomright: (f32,f32)) -> GlMesh {
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

        GlMesh::new(vertices, indices)
    }

    pub fn create_from_pos_size(pos: Vector2<f64>, size: Vector2<f64>) -> GlMesh {
        let top_left = (pos.x as f32, pos.y as f32);
        let bottom_right = (pos.x as f32 + size.x as f32, pos.y as f32 + size.y as f32);
        GlMesh::create_from_topleft_bottomright(top_left, bottom_right)
    }

    pub fn create_quad() -> GlMesh {
        Self::create_rect(1.0,1.0)
    }

    fn draw(&self) {
        unsafe {

        }
    }
}
*/
