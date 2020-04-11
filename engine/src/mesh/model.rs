#![allow(dead_code, unused_variables)]

use genmesh::{
    generators::{IndexedPolygon, SphereUv},
    Triangulate, Vertex,
};
use glm::*;
use itertools::Itertools;
use utils::file::path_of;

#[repr(C)]
pub struct VertexData {
    pub vertex: Vertex,
    pub normal: Vec3,
    pub tex_coord: Vec2,
}

pub struct MemModel {
    pub name: String,
}

#[derive(Clone)]
pub struct Model {
    pub name: String,
    pub num_indices: i32,
}

impl Model {
    pub fn plane() {}
    pub fn create_box() {}
}
