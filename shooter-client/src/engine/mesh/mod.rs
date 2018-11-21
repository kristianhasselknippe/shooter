pub mod model;
pub mod wavefront;
use gl::types::*;
use na::Vector3;

pub type Vertex3 = Vector3<GLfloat>;
pub type Normal = Vertex3;
pub type TexCoord = Vector3<GLfloat>;
