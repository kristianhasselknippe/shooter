#![allow(dead_code, unused_variables)]

extern crate wavefront_obj;

use gl;
use gl::types::*;
use self::wavefront_obj::obj;
use super::{Normal, Vertex3, TexCoord};
use utils::file::read_asset;
use utils::gl::*;
use na::Vector3;
use mesh::wavefront::parse_wavefront;

#[repr(C)]
pub struct VertexData {
    pub vertex: Vertex3,
    pub normal: Normal,
    pub tex_coord: TexCoord,
}

pub struct MemModel {
    pub name: String,
    pub vertex_data: Vec<VertexData>,
    pub indices: Vec<GLuint>,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,

    pub num_indices: i32,
    pub index_type: GLenum,
    vbo: Buffer,
    ebo: Buffer,
}

impl Model {
    pub fn load_from_wavefront_file(name: &str) -> Result<Model, ()> {
        let content = read_asset(name)?;
        let mut mm = parse_wavefront(&content);

        println!("Size of vertex data: {}", ::std::mem::size_of::<VertexData>());

        println!(
            "Number of vertices in model: {}, bytes: {}",
            mm.vertex_data.len(),
            (mm.vertex_data.len() * ::std::mem::size_of::<VertexData>())
        );

        let mut vbo = gen_vertex_array_buffer();
        vbo.bind();
        vbo.upload_data(
            mm.vertex_data.as_ptr() as _,
            (mm.vertex_data.len() * ::std::mem::size_of::<VertexData>()) as _,
        );
        check_gl_errors();
        vbo.unbind();

        let indices = mm.indices;

        println!("Indices length: {}", indices.len());
        println!(
            "Indices lenght bytes: {}",
            (indices.len() * ::std::mem::size_of::<GLuint>()) as isize
        );


        let mut ebo = gen_element_array_buffer();
        ebo.bind();

        ebo.upload_data(
            indices.as_ptr() as _,
            (indices.len() * ::std::mem::size_of::<GLuint>()) as _,
        );
        ebo.unbind();

        Ok(Model {
            name: "Named not handled".to_string(),
            num_indices: indices.len() as i32,
            index_type: gl::UNSIGNED_INT,
            vbo: vbo,
            ebo: ebo,
        })
    }
    pub fn bind(&mut self) {
        self.ebo.bind();
        self.vbo.bind();
    }

    pub fn unbind(&mut self) {
        self.ebo.unbind();
        self.vbo.unbind();
    }
}
