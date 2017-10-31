extern crate xml;

use std;
use std::io::prelude::*;
use std::io::BufReader;


use self::xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Vec2(f64,f64),
    Vec3(f64,f64,f64),
    Vec4(f64,f64,f64,f64),
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: AttributeValue,
}

impl Attribute {
    pub fn from_value_string(name: &str, val: &str) -> Attribute {
        let mut is_number = true;
        if val.contains(",") { //might be a vector
            let components: Vec<&str> = val.split(",").collect();
            let mut parsed_comps = Vec::new();
            'vector: for c in components {
                if let Ok(n) = c.parse::<f64>() {
                    parsed_comps.push(n);
                } else {
                    is_number = false;
                    break 'vector;
                }
            }
            if is_number {
                return Attribute {
                    name: name.to_string(),
                    value: match parsed_comps.len() {
                        1 => AttributeValue::Number(parsed_comps[0]),
                        2 => AttributeValue::Vec2(parsed_comps[0],parsed_comps[1]),
                        3 => AttributeValue::Vec3(parsed_comps[0],parsed_comps[1],parsed_comps[2]),
                        4 => AttributeValue::Vec4(parsed_comps[0],parsed_comps[1],parsed_comps[2],parsed_comps[3]),
                        _ => panic!("Invalid number of components to vector"),
                    }
                }
            }
        }

        //String is the last option
        Attribute {
            name: name.to_string(),
            value: AttributeValue::String(val.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
}

pub fn load_from_file(path: &std::path::Path) -> Result<Node,()> {
    let mut file = std::fs::File::open(std::path::Path::new("scenes/scene1")).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut document = Vec::new();

    let mut scene = None;
    
    for e in parser {
        
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut n = Node { name: name.local_name, attributes: Vec::new(), children: Vec::new() };

                for a in attributes {
                    n.attributes.push(Attribute::from_value_string(&a.name.local_name, &a.value));
                }
                document.push(n);
            },
            Ok(XmlEvent::EndElement { name }) => {
                let x = document.pop().unwrap();
                if let Some(p) = document.last_mut() {
                    p.children.push(x);
                } else {
                    scene = Some(x);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    if let Some(scene) = scene {
        //println!("Scene: {:#?}", scene);
        Ok(scene)
    } else {
        Err(())
    }
}



