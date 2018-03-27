pub mod model;
pub mod mesh;

#[repr(C)]
pub struct Vertex<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vertex<T> {
    fn new(x: T, y: T, z: T) -> Vertex<T> {
        Vertex {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[derive(Debug)]
pub struct Face(i32,i32,i32);
