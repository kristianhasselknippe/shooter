extern crate wavefront_obj;

use gl;
use gl::types::*;
use self::wavefront_obj::obj;
use utils::file::read_asset;
use utils::gl::*;
use super::{Vertex, Face};

#[derive(Debug)]
pub enum Shape {
    Triangle(Face),
}

pub struct Geometry {
    shapes: Vec<Shape>,
}

pub struct Model {
    name: String,

    num_indices: i32,
    vbo: Buffer,
    ebo: Buffer,
}

impl Model {
    pub fn load_from_wavefront_file(name: &str) -> Result<Vec<Model>, ()> {
        println!("Loading wavefront file for : {}", name);
        let content = read_asset(name)?;
        if let Ok(obj) = obj::parse(content) {
            let ret = obj.objects
                .iter()
                .map(|o| {
                    let mut vertices = Vec::new();
                    for v in &o.vertices {
                        vertices.push(v.x as GLfloat);
                        vertices.push(v.y as GLfloat);
                        vertices.push(v.z as GLfloat);
                    }

                    let mut vbo = gen_vertex_array_buffer();
                    vbo.bind();
                    vbo.upload_data(vertices.as_ptr() as _,
                                    (vertices.len() * ::std::mem::size_of::<GLfloat>()) as _);
                    vbo.unbind();

                    let mut indices = Vec::new();
                    for g in &o.geometry {
                        for s in &g.shapes {
                            match s.primitive {
                                wavefront_obj::obj::Primitive::Triangle(a, b, c) => {
                                    indices.push(a.0 as GLuint);
                                    indices.push(b.0 as GLuint);
                                    indices.push(c.0 as GLuint);
                                }
                                _ => panic!("Unsupported shape primitive"),
                            }
                        }
                    }
                    println!("Indices length: {}", indices.len());
                    println!("Indices lenght bytes: {}",
                             (indices.len() * ::std::mem::size_of::<GLfloat>()) as isize);

                    let mut ebo = gen_element_array_buffer();
                    ebo.bind();
                    ebo.upload_data(indices.as_ptr() as _,
                                    (indices.len() * ::std::mem::size_of::<GLuint>()) as isize);
                    ebo.unbind();

                    Model {
                        name: o.name.to_string(),
                        num_indices: indices.len() as i32,
                        vbo: vbo,
                        ebo: ebo,
                    }
                })
                .collect();
            Ok(ret)
        } else {
            Err(())
        }
    }

    pub fn draw(&mut self) {
        self.ebo.bind();
        self.vbo.bind();
        println!("Drawing num indices: {}", self.num_indices);
        unsafe {
            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    3, // Tightly packed atm
                                    0 as *const GLvoid);
            gl::EnableVertexAttribArray(0);
        }
        draw_triangles(self.num_indices as _, gl::UNSIGNED_INT);
        self.ebo.unbind();
        self.vbo.unbind();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_al_model() {
        if let Ok(models) = Model::load_from_wavefront_file("al.obj") {
        } else {
            panic!("Model did not load correctly")
        }
    }
}
