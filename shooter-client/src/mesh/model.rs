extern crate wavefront_obj;

use utils::gl::*;
use self::wavefront_obj::obj;
use utils::file::read_asset;
use super::{Vertex,Face};

#[derive(Debug)]
pub enum Shape {
    Triangle(Face),
}

pub struct Geometry {
    shapes: Vec<Shape>,
}

pub struct Model {
    name: String,
    vertices: Vec<Vertex<f64>>,
    geometry: Vec<Geometry>,
}

impl Model {
    pub fn load_from_wavefront_file(name: &str) -> Result<Vec<Model>,()> {
        println!("Loading wavefront file for : {}", name);
        let content = read_asset(name)?;
        if let Ok(obj) = obj::parse(content) {
            let ret = obj.objects.iter().map(|o| {
                Model {
                    name: o.name.to_string(),
                    vertices: o.vertices.iter().map(|v| Vertex::new(v.x, v.y, v.z)).collect(),
                    geometry: o.geometry.iter().map(|g| Geometry {
                        shapes: g.shapes.iter().map(|s| {
                            match s.primitive {
                                self::wavefront_obj::obj::Primitive::Triangle(a,b,c) => {
                                    Shape::Triangle(Face(a.0 as _, b.0 as _, c.0 as _))
                                },
                                _ => { panic!("Unsupported shape primitive") }
                            }
                        }).collect()
                    }).collect()
                }
            }).collect();
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn get_vertices_byte_ptr(&self) -> *const u8 {
        unsafe {
            self.vertices.as_ptr() as *const u8
        }
    }

    pub fn upload(&mut self) {
        let mut vbo = gen_vertex_array_buffer();
        let mut ebo = gen_element_array_buffer();

        vbo.bind();
        ebo.bind();

        let vertices_bytes_len = self.vertices.len() * ::std::mem::size_of::<f64>();
        let vertices = self.get_vertices_byte_ptr();
        vbo.upload_data(vertices, vertices_bytes_len as _);
        //ebo.upload_data();

        //vbo.enable_vertex_attrib(VertexAttribute::new(0, GLDataType::Float, 3));
        //vbo.enable_vertex_attrib(VertexAttribute::new(1, GLDataType::Float, 2));
            
        /*let mut positions = 
        positions.enable(0,5);
        ux_coords.enable(3,5);*/

        /*gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * self.vertices.len()) as isize,
        mem::transmute(self.vertices.first().unwrap()), gl::STATIC_DRAW);

        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
        mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);

        gl::DrawElements(gl::TRIANGLES, self.n_elements as i32, gl::UNSIGNED_INT, ptr::null());*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn load_al_model() {
        if let Ok(models) = Model::load_from_wavefront_file("al.obj") {
            /*println!("Models:");
            for m in  models {
                println!("\tModel name : {}", m.name);
                for g in m.geometry {
                    println!("\t\tShape:");
                    for t in g.shapes {
                        println!("\t\t\tTriangle: {:?}", t)
                    }
                }
        }*/
        } else {
            panic!("Model did not load correctly")
        }
    }
}
