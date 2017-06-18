use super::transform::*;
use super::mesh::*;

pub struct Entity {
    transform: Transform,
    mesh: Mesh,
}

impl Entity {
    pub fn new_sprite() -> Entity {
        Entity {
            transform: Transform::new(),
            mesh: Mesh::create_quad(),
        }
    }
}
