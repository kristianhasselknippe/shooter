extern crate combine;

use combine::{many, Parser};
use combine::char::letter;

use std;
use std::io::prelude::*;

#[derive(Deserialize,Debug)]
struct Entity {
    Name: String,
}

#[derive(Deserialize,Debug)]
struct Scene {
    #[serde(rename = "Entity", default)]
    children: Vec<Entity>,
}

pub fn load_from_file(path: &std::path::Path) {
    let mut file = std::fs::File::open(std::path::Path::new("scenes/scene1")).unwrap();
    let mut xml = String::new();
    file.read_to_string(&mut xml);
    println!("XML: {}", xml);

    let result = many(letter()).parse("<Scene>");

    println!("Res: {:?}", result);
    
    //let scene: Scene = 

    //println!("{:#?}", scene);
}
