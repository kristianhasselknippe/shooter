#![allow(dead_code, unused_variables)]

extern crate wavefront_obj;

use super::{Normal, TexCoord, Vertex3};
use glm::*;
use itertools::Itertools;
use mesh::wavefront::{parse_wavefront, MtlItem};
use utils::file::path_of;

#[repr(C)]
pub struct VertexData {
    pub vertex: Vertex3,
    pub normal: Normal,
    pub tex_coord: TexCoord,
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
    pub fn load_from_wavefront_file(name: &str) -> Result<Model, ()> {
        unimplemented!();
    }

    pub fn bind(&mut self) {
        unimplemented!();
    }

    pub fn unbind(&mut self) {
        unimplemented!();
    }
}
