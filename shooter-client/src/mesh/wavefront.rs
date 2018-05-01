#![allow(dead_code,unused_variables)]

use gl::types::*;
use super::{Vertex3,Normal,TexCoord};
use na::{Vector3};
use super::model::{ MemModel, Group };

struct WavefrontParser {
    vertices: Vec<Vertex3>,
    normals: Vec<Normal>,
    groups: Vec<Group>
}

impl WavefrontParser {
    fn current_group(&mut self) -> &mut Group {
        let index = self.groups.len() - 1;
        &mut self.groups[index]
    }

    fn split_parts<'a>(&mut self, content: &'a str) -> Vec<&'a str> {
        content.split(|c| c == ' ' || c == ',').collect()
    }

    fn parse_obj(&mut self, line: &str) {
        self.parse_group(line);
    }

    fn parse_group(&mut self, line: &str) {
        //println!("Parsed group: {}", line);
        let split = self.split_parts(line);
        let name = if split.len() > 1 { split[1] } else { "default" };
        self.groups.push(Group {
            name: name.to_string(),
            indices: Vec::new(),
        })
    }

    fn split_parts_f32(&mut self, content: &str) -> Vec<f32> {
        self.split_parts(content).iter().map(|p| {
            let trimmed = p.trim();
            if let Ok(ret) = trimmed.parse::<f32>() {
                ret
            } else {
                panic!("Unable to parse float, {:?}", trimmed);
            }
        }).collect()
    }

    fn parse_vert(&mut self, line: &str) {
        let parts = self.split_parts_f32(line);
        let len = parts.len();
        match len {
            3 => {
                let vertex = Vertex3::new(parts[0], parts[1], parts[2]);
                //println!("Vertex: {:?}", vertex);
                self.vertices.push(vertex);
            },
            _ => {
                panic!("Vertex has more components than we currently handle: {}", len);
            }
        }
    }

    fn parse_texcoord(&mut self, line: &str) {
        let parts = self.split_parts_f32(line);
        let len = parts.len();
        match len {
            2 => {
                let texcoord = TexCoord::new(parts[0], parts[1]);
                //println!("TexCoord: {:?}", texcoord);
            },
            _ => {
                panic!("Texcoord has more components than we handle: {}", len);
            }
        }
    }

    fn parse_face(&mut self, line: &str) {
        let parts: Vec<(f32,Option<f32>,Option<f32>)> = self.split_parts(line)
            .iter()
            .map(|x| {
                let split: Vec<&str> = x.split(|c| c == '/').collect();
                match split.len() {
                    1 => {
                        (split[0].parse::<f32>().unwrap(), None, None)
                    },
                    2 => {
                        (split[0].parse::<f32>().unwrap(),
                         split[1].parse::<f32>().map(|x| Some(x)).unwrap_or(None),
                         None)
                    },
                    3 => {
                        (split[0].parse::<f32>().unwrap(),
                         split[1].parse::<f32>().map(|x| Some(x)).unwrap_or(None),
                         split[2].parse::<f32>().map(|x| Some(x)).unwrap_or(None))
                    },
                    _ => { panic!("Faces can't have more than 3 items per value"); }
                }
            })
            .collect();
        
        if parts.len() <= 2 {
            panic!("Faces should refer to 3 or more vertices")
        }

        for i in 0..parts.len() - 2 {
            self.current_group().indices.push(parts[0].0 as u32 - 1);
            self.current_group().indices.push(parts[i + 1].0 as u32 - 1);
            self.current_group().indices.push(parts[i + 2].0 as u32 - 1);
        }
    }

    fn parse_s(&mut self, line: &str) {

    }

    fn parse_usemtl(&mut self, line: &str) {

    }

    fn match_line_type(&mut self) {

    }

    fn parse_line(&mut self, line: &str) {
        //println!("Parsing line: {}", line);
        let mut end = 0;
        let ref mut line = if line.len() > 1 {
            line.to_string()
        } else {
            line.to_owned() + " "
        };
        for c in line.chars() {
            match c {
                ' ' | ',' => {
                    let subline = &line[0..end];
                    //println!("Line: {}, Subline: {}", line, subline);
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
                        "g" => {
                            self.parse_group(&line[end..line.len() as usize]);
                        },
                        "#" => {
                            //println!("Comment {}", &line[end..line.len() as usize]);
                        },
                        "usemtl" => {
                            self.parse_usemtl(&line[end..line.len() as usize]);
                        },
                        &_ => {
                            //println!("[Wavefront]: Unrecognized line qualifier: {}", subline);
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

pub fn parse_wavefront(content: &str) -> MemModel {
    let mut parser = WavefrontParser {
        vertices: Vec::new(),
        normals: Vec::new(),
        groups: Vec::new(),
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

    let mut normals = vec![Normal::new(0.0,0.0,0.0);parser.vertices.len()];

    for g in &parser.groups {
        let mut i = 0;
        while i < g.indices.len() {
            let a = &parser.vertices[g.indices[i] as usize];
            let a = Vector3::new(a.x,a.y,a.z);
            let b = &parser.vertices[g.indices[i+1] as usize];
            let b = Vector3::new(b.x,b.y,b.z);
            let c = &parser.vertices[g.indices[i+2] as usize];
            let c = Vector3::new(c.x,c.y,c.z);

            let v1 = a - b;
            let v2 = c - a;

            let normal = v1.cross(&v2);
            normals[g.indices[i] as usize] += normal;
            normals[g.indices[i+1] as usize] += normal;
            normals[g.indices[i+2] as usize] += normal;

            i += 3;
        }
    }

    for i in 0..normals.len() {
        normals[i] = normals[i].normalize();
    }

    //println!("Normals: {:#?}", normals);

    for v in &parser.vertices {

    }

    MemModel {
        name: "No name yet".to_string(),
        vertices: parser.vertices,
        normals: normals,
        groups: parser.groups,
    }
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
