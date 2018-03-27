pub mod model;
pub mod mesh;

use na::core::Vector3;

pub type Vertex<T> = Vector3<T>;

#[derive(Debug)]
pub struct Face(i32,i32,i32);
