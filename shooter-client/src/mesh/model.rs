#![allow(dead_code,unused_variables)]

extern crate wavefront_obj;

use gl;
use gl::types::*;
use self::wavefront_obj::obj;
use super::{Vertex3,Normal};
use utils::file::read_asset;
use utils::gl::*;
use na::{Vector3};

pub struct MemModel {
    vertices: Vec<Vertex3>,
    normals: Vec<Normal>,
    indices: Vec<GLuint>,
}

impl MemModel {
    pub fn new(vertices: Vec<Vertex3>,
               normals: Vec<Normal>,
               indices: Vec<GLuint>,) -> MemModel {
        MemModel {
            vertices: vertices,
            normals: normals,
            indices: indices,
        }
    }
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
    pub fn load_from_wavefront_file(name: &str) -> Result<Vec<Model>, ()> {
        println!("Loading wavefront file for : {}", name);
        let content = read_asset(name)?;
        match obj::parse(content) {
            Ok(obj) => {
                let ret = obj.objects
                    .iter()
                    .map(|o| {
                        let stride = 3+3;//+2;//vertices(3) normals(3) texcoords(2)
                        let mut vertices: Vec<GLfloat> = vec![0.0;o.vertices.len() * stride];
                        
                        println!("Vertices: {}", vertices.len());
                        

                        let mut indices = Vec::new();
                        for g in &o.geometry {
                            for s in &g.shapes {
                                match s.primitive {
                                    wavefront_obj::obj::Primitive::Triangle(a, b, c) => {
                                        indices.push(a.0 as GLuint);
                                        indices.push(b.0 as GLuint);
                                        indices.push(c.0 as GLuint);

                                        let base_index_a = a.0 * stride;
                                        vertices[base_index_a] = o.vertices[a.0].x as GLfloat;
                                        vertices[base_index_a+1] = o.vertices[a.0].y as GLfloat;
                                        vertices[base_index_a+2] = o.vertices[a.0].z as GLfloat;

                                        let base_index_b = b.0 * stride;
                                        vertices[base_index_b] = o.vertices[b.0].x as GLfloat;
                                        vertices[base_index_b+1] = o.vertices[b.0].y as GLfloat;
                                        vertices[base_index_b+2] = o.vertices[b.0].z as GLfloat;

                                        
                                        let base_index_c = c.0 * stride;
                                        vertices[base_index_c] = o.vertices[c.0].x as GLfloat;
                                        vertices[base_index_c+1] = o.vertices[c.0].y as GLfloat;
                                        vertices[base_index_c+2] = o.vertices[c.0].z as GLfloat;
                                        

                                        //Normal coords
                                        if let (Some(na),Some(nb),Some(nc)) = (a.2,b.2,c.2) {
                                            
                                        } else {
                                            let _va = o.vertices[a.0];
                                            let _vb = o.vertices[b.0];
                                            let _vc = o.vertices[c.0];
                                            let va = Vector3::new(_va.x, _va.y, _va.z);
                                            let vb = Vector3::new(_vb.x, _vb.y, _vb.z);
                                            let vc = Vector3::new(_vc.x, _vc.y, _vc.z);
                                            let dir1 = vc - vb;
                                            let dir2 = vc - va;
                                            let normal = dir1.cross(&dir2).normalize();
                                            
                                            vertices[base_index_a+3] += normal.x as GLfloat;
                                            vertices[base_index_a+4] += normal.y as GLfloat;
                                            vertices[base_index_a+5] += normal.z as GLfloat;
                                            
                                            vertices[base_index_b+3] += normal.x as GLfloat;
                                            vertices[base_index_b+4] += normal.y as GLfloat;
                                            vertices[base_index_b+5] += normal.z as GLfloat;
                                           
                                            vertices[base_index_c+3] += normal.x as GLfloat;
                                            vertices[base_index_c+4] += normal.y as GLfloat;
                                            vertices[base_index_c+5] += normal.z as GLfloat;
                                        }
                                        
                                        //Texture coords
                                        /*if let (Some(ta),Some(tb),Some(tc)) = (a.1,b.1,c.1) {
                                            let base_index_a = ta * stride;
                                            vertices[base_index_a+6] = o.vertices[na].y;
                                            vertices[base_index_a+7] = o.vertices[na].z;
                                            let base_index_a = ta * stride;
                                            vertices[base_index_a+6] = o.vertices[na].y;
                                            vertices[base_index_a+7] = o.vertices[na].z;
                                            let base_index_a = ta * stride;
                                            vertices[base_index_a+6] = o.vertices[na].y;
                                            vertices[base_index_a+7] = o.vertices[na].z;
                                        }*/
                                    }
                                    _ => panic!("Unsupported shape primitive"),
                                }
                            }
                        }

                        for i in 0..o.vertices.len() {
                            vertices[(i * stride) + 3] /= 3.0;
                            vertices[(i * stride) + 4] /= 3.0;
                            vertices[(i * stride) + 5] /= 3.0;
                        }

                        
                        let mut vbo = gen_vertex_array_buffer();
                        vbo.bind();
                        vbo.upload_data(vertices.as_ptr() as _,
                                        (vertices.len() * ::std::mem::size_of::<GLfloat>()) as _);
                        check_gl_errors();
                        vbo.unbind();
                        
                        println!("Indices length: {}", indices.len());
                        println!("Indices lenght bytes: {}",
                                 (indices.len() * ::std::mem::size_of::<GLuint>()) as isize);

                        let mut ebo = gen_element_array_buffer();
                        ebo.bind();
                        ebo.upload_data(indices.as_ptr() as _,
                                        (indices.len() * ::std::mem::size_of::<GLuint>()) as _);
                        ebo.unbind();

                        Model {
                            name: o.name.to_string(),
                            num_indices: indices.len() as i32,
                            index_type: gl::UNSIGNED_INT,
                            vbo: vbo,
                            ebo: ebo,
                        }
                    })
                    .collect();
                Ok(ret)
            },
            Err(e) => {
                println!("Error loading obj file: {:?}", e);
                Err(())
            }
        }
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
