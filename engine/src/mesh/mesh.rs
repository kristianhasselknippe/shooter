use glm::{vec3, Vec3, rotate_vec3};
use num::Num;
use std::f32::consts::PI;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
}

impl Mesh {

    fn rotated_around_z(&self, angle: f32) -> Mesh {
        Mesh {
            vertices: self.vertices.iter().map(|v| {
                rotate_vec3(v, angle, &vec3(0.0,0.0,1.0))
            }).collect(),
            normals: self.normals.iter().cloned().collect(),
            indices: self.indices.iter().cloned().collect(),
        }
    }

    pub fn create_triangle() -> Mesh {
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

    pub fn create_square() -> Mesh {
        let t1 = Self::create_triangle();
        let t2 = Self::create_triangle().rotated_around_z(PI / 2.0);

        let t1_indices_len = t1.indices.len() as u32;

        return Mesh {
            vertices: [t1.vertices, t2.vertices].concat(),
            normals: [t1.normals, t2.normals].concat(),
            indices: [t1.indices, t2.indices.iter().map(|e| (e + t1_indices_len) as u32).collect()].concat(),
        }
    }

    pub fn create_box() -> Mesh {
        unimplemented!();
    }
}
