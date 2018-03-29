pub mod model;
pub mod wavefront;
use gl::types::*;

#[derive(Debug)]
#[repr(C)]
pub struct TexCoord {
    u: GLfloat,
    v: GLfloat,
}

impl TexCoord {
    pub fn new(u: GLfloat, v: GLfloat) -> TexCoord {
        TexCoord {
            u: u,
            v: v,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Vertex3 {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
}

#[derive(Debug)]
#[repr(C)]
pub struct Vertex4 {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
}

pub type Normal = Vertex3;

impl Vertex3 {
    fn new(x: GLfloat, y: GLfloat, z: GLfloat) -> Vertex3 {
        Vertex3 {
            x: x,
            y: y,
            z: z,
        }
    }
}
