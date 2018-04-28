#![allow(dead_code,unused_variables)]

use utils::file::*;
use gl::types::*;
use super::{Vertex3,Normal,TexCoord};
use super::model::MemModel;


struct WavefrontModel {
    object: MemModel
}

impl WavefrontParser {

    fn parse_obj(&mut self, line: &str) {
        println!("Parsed obj: {}", line);
    }

    fn split_parts(&mut self, content: &str) -> Vec<&str> {
        content.split(|c| c == ' ' || c == ',').collect()
    }

    fn split_parts_f32(&mut self, content: &str) -> Vec<f32> {
        split_parts(content).iter().map(|p| {
            let trimmed = p.trim();
            if let Ok(ret) = trimmed.parse::<f32>() {
                ret
            } else {
                panic!("Unable to parse float, {:?}", trimmed);
            }
        }).collect()
    }

    fn parse_vert(&mut self, line: &str) {
        let parts = split_parts_f32(line);
        let len = parts.len();
        match len {
            3 => {
                let vertex = Vertex3::new(parts[0], parts[1], parts[2]);
                println!("Vertex: {:?}", vertex);
            },
            _ => {
                panic!("Vertex has more components than we currently handle: {}", len);
            }
        }
    }

    fn parse_texcoord(&mut self, line: &str) {
        let parts = split_parts_f32(line);
        let len = parts.len();
        match len {
            2 => {
                let texcoord = TexCoord::new(parts[0], parts[1]);
                println!("TexCoord: {:?}", texcoord);
            },
            _ => {
                panic!("Texcoord has more components than we handle: {}", len);
            }
        }
    }

    fn parse_face(&mut self, line: &str) {

    }

    fn parse_s(&mut self, line: &str) {

    }

    fn parse_usemtl(&mut self, line: &str) {

    }

    fn parse_line(&mut self, line: &str) {
        println!("Parsing line: {}", line);
        let mut end = 0;
        for c in line.chars() {
            match c {
                ' ' | ',' => {
                    let subline = &line[0..end];
                    end += 1;
                    match subline {
                        "v" => {
                            self.parse_vert(&line[end..line.len() as usize]);
                        },
                        "vt" => {
                            self.parse_texcoord(&line[end..line.len() as usize]);
                        },
                        "f" => {
                            self.parse_face(&line[end..line.len() as usize]);
                        },
                        "s" => {
                            self.parse_s(&line[end..line.len() as usize]);
                        },
                        "o" => {
                            self.parse_obj(&line[end..line.len() as usize]);
                        },
                        "#" => {
                            println!("Comment {}", &line[end..line.len() as usize]);
                        },
                        "usemtl" => {
                            self.parse_usemtl(&line[end..line.len() as usize]);
                        },
                        &_ => {
                            panic!("[Wavefront]: Unrecognized line qualifier: {}", subline);
                        }
                    }
                    break;
                },
                _ => {}
            }
            end += 1;
        }
    }
}

struct WavefrontParser {
    vertices: Vec<Vertex3>,
    normals: Vec<Normal>,
    indices: Vec<GLuint>,
}

pub fn parse_wavefront(content: &str) -> MemModel {
    let mut parser = WavefrontParser {
        vertices: Vec::new(),
        normals: Vec::new(),
        indices: Vec::new(),
    };

    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    for c in content.chars() {
        match c {
            '\n' => {
                parser.parse_line(&content[line_start..line_end]);
                line_start = line_end + 1;
            },
            _ => { }
        }
        line_end += 1;
    }

    MemModel::new( vertices, normals, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_quad_model() {
        let asset = read_asset("quad.obj").unwrap();
        parse_wavefront(&asset);
    }
}
