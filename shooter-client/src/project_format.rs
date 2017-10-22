extern crate xml;

use std;
use std::io::prelude::*;
use std::io::BufReader;


use self::xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
enum AttributeValue {
    String(String),
}

#[derive(Debug)]
struct Attribute {
    name: String,
    value: AttributeValue,
}

#[derive(Debug)]
struct Node {
    name: String,
    attributes: Vec<Attribute>
}

pub fn load_from_file(path: &std::path::Path) {
    let mut file = std::fs::File::open(std::path::Path::new("scenes/scene1")).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut document = Vec::new();
    
    for e in parser {
        
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let mut n = Node { name: name.local_name, attributes: Vec::new() };

                for a in attributes {
                    n.attributes.push(Attribute {
                        name: a.name.local_name,
                        value: AttributeValue::String(a.value),
                    });
                }
                document.push(n);
            },
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}



