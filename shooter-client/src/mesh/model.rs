#![allow(dead_code, unused_variables)]

extern crate wavefront_obj;

use gl;
use gl::types::*;
use self::wavefront_obj::obj;
use super::{Normal, Vertex3};
use utils::file::read_asset;
use utils::gl::*;
use na::Vector3;
use mesh::wavefront::parse_wavefront;

pub struct Group {
    pub name: String,
    pub indices: Vec<GLuint>,
}

pub struct MemModel {
    pub name: String,
    pub vertices: Vec<Vertex3>,
    pub normals: Vec<Normal>,
    pub groups: Vec<Group>,
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
        let mut vertices = Vec::with_capacity(mm.vertices.len() + mm.normals.len());

        for i in 0..mm.vertices.len() {
            vertices.push(mm.vertices[i]);
            vertices.push(mm.normals[i]);
        }

        println!(
            "Number of vertices in model: {}, bytes: {}",
            vertices.len(),
            (vertices.len() * ::std::mem::size_of::<Vertex3>())
        );



        let mut vbo = gen_vertex_array_buffer();
        vbo.bind();
        vbo.upload_data(
            vertices.as_ptr() as _,
            (vertices.len() * ::std::mem::size_of::<Vertex3>()) as _,
        );
        check_gl_errors();
        vbo.unbind();

        //let normals = g.normals;
        let mut indices = Vec::new();
        for g in &mut mm.groups {
            indices.append(&mut g.indices);
        }

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
