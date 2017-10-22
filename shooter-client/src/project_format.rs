extern crate xml;

use std;
use std::io::prelude::*;
use std::io::BufReader;


use self::xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub enum AttributeValue {
    String(String),
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: AttributeValue,
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
                    n.attributes.push(Attribute {
                        name: a.name.local_name,
                        value: AttributeValue::String(a.value),
                    });
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
        Ok(scene)
    } else {
        Err(())
    }
}



