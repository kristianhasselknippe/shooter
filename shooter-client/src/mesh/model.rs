extern crate wavefront_obj;

use self::wavefront_obj::obj;
use utils::file::read_asset;
use super::Vertex;
use std::io::Error;

pub struct Object {
}

pub struct Model {
    name: String,
    vertices: Vec<Vertex<f64>>,
    //objects: Vec<Object>,
}

impl Model {
    fn load_from_wavefront_file(name: &str) -> Result<Vec<Model>,()> {
        println!("Loading wavefront file for : {}", name);
        let content = read_asset(name)?;
        if let Ok(obj) = obj::parse(content) {
            let ret = obj.objects.iter().map(|o| {
                Model {
                    name: o.name.clone(),
                    vertices: o.vertices.iter().map(|v| Vertex::new(v.x, v.y, v.z)).collect()
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
        if let Ok(model) = Model::load_from_wavefront_file("al.obj") {
            
        } else {
            panic!("Model did not load correctly")
        }
    }
}
