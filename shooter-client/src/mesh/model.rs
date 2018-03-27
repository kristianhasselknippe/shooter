extern crate wavefront_obj;

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
    fn load_from_wavefront_file(name: &str) -> Result<Vec<Model>,()> {
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
