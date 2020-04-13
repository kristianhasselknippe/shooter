use glm::{rotate_vec3, vec3, Vec3};
use num::Num;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub colors: Vec<Vec3>,
    pub indices: Vec<u16>,
}

impl Mesh {
    pub fn scaled(&self, x: f32, y: f32, z: f32) -> Mesh {
        Mesh {
            vertices: self
                .vertices
                .iter()
                .map(|v| vec3(v.x * x, v.y * y, v.z * z))
                .collect(),
            normals: self.normals.iter().cloned().collect(),
            colors: self.colors.iter().cloned().collect(),
            indices: self.indices.iter().cloned().collect(),
        }
    }

    pub fn rotated(&self, angle: f32, axis: &Vec3) -> Mesh {
        Mesh {
            vertices: self
                .vertices
                .iter()
                .map(|v| rotate_vec3(v, angle, axis))
                .collect(),
            normals: self.normals.iter().cloned().collect(),
            colors: self.colors.iter().cloned().collect(),
            indices: self.indices.iter().cloned().collect(),
        }
    }

    pub fn rotated_around_x(&self, angle: f32) -> Mesh {
        self.rotated(angle, &vec3(1.0, 0.0, 0.0))
    }

    pub fn rotated_around_y(&self, angle: f32) -> Mesh {
        self.rotated(angle, &vec3(0.0, 1.0, 0.0))
    }

    pub fn rotated_around_z(&self, angle: f32) -> Mesh {
        self.rotated(angle, &vec3(0.0, 0.0, 1.0))
    }

    pub fn translated(&self, x: f32, y: f32, z: f32) -> Mesh {
        Mesh {
            vertices: self
                .vertices
                .iter()
                .map(|v| vec3(v.x + x, v.y + y, v.z + z))
                .collect(),
            normals: self.normals.iter().cloned().collect(),
            colors: self.colors.iter().cloned().collect(),
            indices: self.indices.iter().cloned().collect(),
        }
    }

    pub fn combine(&self, other: &Mesh) -> Mesh {
        let m1 = self.clone();
        let m2 = other.clone();

        let m1_indices_len = m1.indices.len() as u16;

        Mesh {
            vertices: [m1.vertices, m2.vertices].concat(),
            normals: [m1.normals, m2.normals].concat(),
            colors: [m1.colors, m2.colors].concat(),
            indices: [
                m1.indices,
                m2.indices
                    .iter()
                    .map(|e| (e + m1_indices_len) as u16)
                    .collect(),
            ]
            .concat(),
        }
    }

    pub fn with_color(&self, color: &Vec3) -> Mesh {
        Mesh {
            vertices: self.vertices.iter().cloned().collect(),
            normals: self.normals.iter().cloned().collect(),
            colors: std::vec::from_elem(color.clone(), self.vertices.len()),
            indices: self.indices.iter().cloned().collect(),
        }
    }

    pub fn create_triangle(color: &Option<Vec3>) -> Mesh {
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
            colors: {
                if let &Some(color) = color {
                    std::vec::from_elem(color, 3)
                } else {
                    std::vec::from_elem(vec3(1.0, 0.0, 1.0), 3)
                }
            },
            indices: vec![0, 1, 2],
        };
    }

    pub fn create_square(color: &Option<Vec3>) -> Mesh {
        let t1 = Self::create_triangle(color);
        let t2 = Self::create_triangle(color)
            .rotated_around_z(PI / 2.0)
            .scaled(-1.0, 1.0, 1.0);
        t1.combine(&t2)
    }

    pub fn create_box(color: &Option<Vec3>) -> Mesh {
        Self::create_square(color)
            //.combine(&Self::create_square().translated(0.0, 0.0, 1.0))
            .combine(&Self::create_square(&Some(vec3(0.0, 1.0, 0.0))).rotated_around_y(PI / 2.0))
            .combine(&Self::create_square(&Some(vec3(1.0, 1.0, 0.0))).rotated_around_x(-PI / 2.0))

            .combine(&Self::create_square(&Some(vec3(1.0, 0.0, 0.0))).translated(0.0, 0.0, -1.0))
            .combine(&Self::create_square(&Some(vec3(0.0, 1.0, 1.0))).translated(0.0,0.0,1.0).rotated_around_y(PI / 2.0))
            .combine(&Self::create_square(&Some(vec3(1.0, 1.0, 1.0))).translated(0.0,0.0,1.0).rotated_around_x(-PI / 2.0))
    }
}
