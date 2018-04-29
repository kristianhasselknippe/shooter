pub mod model;
pub mod wavefront;
use gl::types::*;
use std::ops::{Add,Sub,AddAssign};
use na::Vector3;

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

pub type Vertex3 = Vector3<GLfloat>;
pub type Normal = Vertex3;

#[derive(Debug)]
#[repr(C)]
pub struct Vertex4 {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
}


