use glm::{vec3, Vec3};
use num::Num;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn create_square() -> Mesh {
        return Mesh {
            vertices: vec![
                vec3(0.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 1.0, 0.0),
            ],
            normals: vec![
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, 1.0),
            ],
            indices: vec![0, 1, 2],
        };
    }

    pub fn create_box() -> Mesh {
        unimplemented!();
    }
}
