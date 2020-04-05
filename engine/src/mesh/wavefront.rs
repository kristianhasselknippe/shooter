#![allow(dead_code, unused_variables)]

use super::model::{MemModel, VertexData};
use super::{Normal, TexCoord, Vertex3};
use glm::*;
use std::path::Path;
use utils::file::*;

struct FaceItem {
    vertex: i32,
    tex_coord: Option<i32>,
    normal: Option<i32>,
}

struct Face {
    items: Vec<FaceItem>,
}

pub struct Group {
    pub name: String,
    pub usemtl: Option<String>,

    faces: Vec<Face>,
}

struct WavefrontParser {
    mtl_path: Option<String>,

    vertices: Vec<Vertex3>,
    normals: Vec<Normal>,
    texture_coords: Vec<TexCoord>,

    groups: Vec<Group>,
}

fn split_parts<'a>(content: &'a str) -> Vec<&'a str> {
    content.split(|c| c == ' ' || c == ',').collect()
}

fn split_parts_f32(content: &str) -> Vec<f32> {
    let content = content.trim();
    split_parts(content)
        .iter()
        .map(|p| {
            let trimmed = p.trim();
            match trimmed.parse::<f32>() {
                Ok(ret) => ret,
                Err(e) => {
                    println!("Error parsing f32: {:#?}", e);
                    panic!(
                        "Unable to parse float for content {}, {:?}",
                        content, trimmed
                    );
                }
            }
        })
        .collect()
}

impl WavefrontParser {
    fn current_group(&mut self) -> &mut Group {
        let index = self.groups.len() - 1;
        &mut self.groups[index]
    }

    fn parse_obj(&mut self, line: &str) {
        self.parse_group(line);
    }

    fn parse_group(&mut self, line: &str) {
        //println!("Parsed group: {}", line);
        let split = split_parts(line);
        let name = if split.len() > 1 { split[1] } else { "default" };
        self.groups.push(Group {
            name: name.to_string(),
            usemtl: None,
            faces: Vec::new(),
        })
    }

    fn parse_vert(&mut self, line: &str) {
        let parts = split_parts_f32(line);
        let len = parts.len();
        match len {
            3 => {
                let vertex = Vertex3::new(parts[0], parts[1], parts[2]);
                //println!("Vertex: {:?}", vertex);
                self.vertices.push(vertex);
            }
            _ => {
                panic!(
                    "Vertex has more components than we currently handle: {}",
                    len
                );
            }
        }
    }

    fn parse_vert_norm(&mut self, line: &str) {
        let parts = split_parts_f32(line);
        let len = parts.len();
        match len {
            3 => {
                let norm = Normal::new(parts[0], parts[1], parts[2]);
                //println!("Vertex: {:?}", vertex);
                self.normals.push(norm);
            }
            _ => {
                panic!(
                    "Vertex has more components than we currently handle: {}",
                    len
                );
            }
        }
    }

    fn parse_texcoord(&mut self, line: &str) {
        let parts = split_parts_f32(line);
        let len = parts.len();
        match len {
            2 => {
                let texcoord = TexCoord::new(parts[0], parts[1], 0.0);
                self.texture_coords.push(texcoord);
                //println!("TexCoord: {:?}", texcoord);
            }
            3 => {
                let texcoord = TexCoord::new(parts[0], parts[1], parts[2]);
                self.texture_coords.push(texcoord);
            }
            _ => {
                panic!("Texcoord has more components than we handle: {}", len);
            }
        }
    }

    fn parse_face(&mut self, line: &str) {
        let face_items: Vec<FaceItem> = split_parts(line)
            .iter()
            .map(|x| {
                let split: Vec<&str> = x.split(|c| c == '/').collect();
                match split.len() {
                    1 => FaceItem {
                        vertex: split[0].parse::<i32>().unwrap(),
                        normal: None,
                        tex_coord: None,
                    },
                    2 => FaceItem {
                        vertex: split[0].parse::<i32>().unwrap(),
                        tex_coord: split[1].parse::<i32>().map(|x| Some(x)).unwrap_or(None),
                        normal: None,
                    },
                    3 => FaceItem {
                        vertex: split[0].parse::<i32>().unwrap(),
                        tex_coord: split[1].parse::<i32>().map(|x| Some(x)).unwrap_or(None),
                        normal: split[2].parse::<i32>().map(|x| Some(x)).unwrap_or(None),
                    },
                    _ => {
                        panic!("Faces can't have more than 3 items per value");
                    }
                }
            })
            .collect();
        self.current_group().faces.push(Face { items: face_items })
    }

    fn parse_s(&mut self, line: &str) {}

    fn parse_mtllib(&mut self, line: &str) {
        self.mtl_path = Some(line.to_string());
    }

    fn parse_usemtl(&mut self, line: &str) {
        let g = self.current_group();
        g.usemtl = Some(line.to_string());
    }

    fn match_line_type(&mut self) {}

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
                        }
                        "vn" => {
                            self.parse_vert_norm(&line[end..line.len() as usize]);
                        }
                        "vt" => {
                            self.parse_texcoord(&line[end..line.len() as usize]);
                        }
                        "f" => {
                            self.parse_face(&line[end..line.len() as usize]);
                        }
                        "s" => {
                            self.parse_s(&line[end..line.len() as usize]);
                        }
                        "o" => {
                            self.parse_obj(&line[end..line.len() as usize]);
                        }
                        "g" => {
                            self.parse_group(&line[end..line.len() as usize]);
                        }
                        "#" => {
                            //println!("Comment {}", &line[end..line.len() as usize]);
                        }
                        "mtllib" => {
                            self.parse_mtllib(&line[end..line.len() as usize]);
                        }
                        "usemtl" => {
                            self.parse_usemtl(&line[end..line.len() as usize]);
                        }
                        &_ => {
                            //println!("[Wavefront]: Unrecognized line qualifier: {}", subline);
                        }
                    }
                    break;
                }
                _ => {}
            }
            end += 1;
        }
    }
}

pub fn parse_wavefront(name: &str) -> MemModel {
    let content = read_asset(name).unwrap();
    let mut parser = WavefrontParser {
        vertices: Vec::new(),
        normals: Vec::new(),
        texture_coords: Vec::new(),
        groups: Vec::new(),
        mtl_path: None,
    };

    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    for c in content.chars() {
        match c {
            '\n' => {
                parser.parse_line(&content[line_start..line_end]);
                line_start = line_end + 1;
            }
            _ => {}
        }
        line_end += 1;
    }

    let mut vertex_data = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut index_offset = 0;

    println!("Parser texcoords: {}", parser.texture_coords.len());

    for g in &parser.groups {
        for f in &g.faces {
            for face_item in &f.items {
                if parser.normals.len() != 0 && parser.texture_coords.len() != 0 {
                    let vd = VertexData {
                        vertex: parser.vertices[(face_item.vertex - 1) as usize],
                        normal: -parser.normals[(face_item.normal.unwrap() - 1) as usize],
                        tex_coord: parser.texture_coords
                            [(face_item.tex_coord.unwrap() - 1) as usize],
                    };
                    vertex_data.push(vd);
                } else {
                    let vd = VertexData {
                        vertex: parser.vertices[(face_item.vertex - 1) as usize],
                        normal: Normal::new(0.0, 0.0, 0.0),
                        tex_coord: TexCoord::new(0.0, 0.0, 0.0),
                    };
                    vertex_data.push(vd);
                }
            }
            for i in 0..f.items.len() - 2 {
                indices.push((index_offset) as u32);
                indices.push((index_offset + i + 1) as u32);
                indices.push((index_offset + i + 2) as u32);
            }
            index_offset += f.items.len();
        }
    }

    let mut index_offset = 0;
    if parser.normals.len() == 0 {
        for g in &parser.groups {
            for f in &g.faces {
                for i in 0..f.items.len() - 2 {
                    let a = vertex_data[(index_offset) as usize].vertex;
                    let b = vertex_data[((index_offset + i + 1) as usize)].vertex;
                    let c = vertex_data[((index_offset + i + 2) as usize)].vertex;

                    let v1 = a - b;
                    let v2 = c - a;

                    let normal = v1.cross(&v2);
                    vertex_data[index_offset as usize].normal += normal;
                    vertex_data[(index_offset + i + 1) as usize].normal += normal;
                    vertex_data[(index_offset + i + 2) as usize].normal += normal;
                }
                index_offset += f.items.len();
            }
        }
    }

    let mut mtl = None;
    if let Some(mtl_path) = parser.mtl_path {
        mtl = Some(parse_mtl(name, &mtl_path));
    }

    MemModel {
        name: "No name yet".to_string(),
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MtlItem {
    pub name: String,
    pub Ns: Option<f32>,
    pub Ka: Option<Vec3>,
    pub Kd: Option<Vec3>,
    pub Ks: Option<Vec3>,
    pub Ni: Option<f32>,
    pub d: Option<f32>,
    pub illum: Option<f32>,
    pub map_Kd: Option<String>,
}

struct MtlParser {
    items: Vec<MtlItem>,
}

impl MtlParser {
    fn current_item(&mut self) -> &mut MtlItem {
        if self.items.len() > 0 {
            let i = self.items.len() - 1;
            return &mut self.items[i];
        } else {
            panic!("Asked for current item, but items len is 0");
        }
    }

    fn parse_f32(&mut self, line: &str) -> f32 {
        let parts = split_parts_f32(line);
        if parts.len() != 1 {
            panic!("Error parsing f32: actual len: {}", parts.len());
        }
        parts[0]
    }

    fn parse_vector2(&mut self, line: &str) -> Vec2 {
        let parts = split_parts_f32(line);
        if parts.len() != 2 {
            panic!("Error parsing vec2: actual len: {}", parts.len());
        }
        vec2(parts[0], parts[1])
    }

    fn parse_vector3(&mut self, line: &str) -> Vec3 {
        let parts = split_parts_f32(line);
        if parts.len() != 3 {
            panic!("Error parsing vec3: actual len: {}", parts.len());
        }
        vec3(parts[0], parts[1], parts[2])
    }

    fn parse_name(&mut self, line: &str) -> String {
        let parts = split_parts(line);
        if parts.len() != 1 {
            panic!("Error parsing name: actual len: {}", parts.len());
        }
        parts[0].to_string()
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
                        "newmtl" => {
                            let name = self.parse_name(&line[end..line.len() as usize]);
                            self.items.push(MtlItem {
                                name: name,
                                Ns: None,
                                Ka: None,
                                Kd: None,
                                Ks: None,
                                Ni: None,
                                d: None,
                                illum: None,
                                map_Kd: None,
                            })
                        }
                        "Ns" => {
                            self.current_item().Ns =
                                Some(self.parse_f32(&line[end..line.len() as usize]));
                        }
                        "Ka" => {
                            self.current_item().Ka =
                                Some(self.parse_vector3(&line[end..line.len() as usize]));
                        }
                        "Kd" => {
                            self.current_item().Kd =
                                Some(self.parse_vector3(&line[end..line.len() as usize]));
                        }
                        "Ks" => {
                            self.current_item().Ks =
                                Some(self.parse_vector3(&line[end..line.len() as usize]));
                        }
                        "Ni" => {
                            self.current_item().Ni =
                                Some(self.parse_f32(&line[end..line.len() as usize]));
                        }
                        "d" => {
                            self.current_item().d =
                                Some(self.parse_f32(&line[end..line.len() as usize]));
                        }
                        "illum" => {
                            self.current_item().illum =
                                Some(self.parse_f32(&line[end..line.len() as usize]));
                        }
                        "map_Kd" => {
                            self.current_item().map_Kd =
                                Some(self.parse_name(&line[end..line.len() as usize]));
                        }
                        &_ => {
                            //println!("[Wavefront]: Unrecognized line qualifier: {}", subline);
                        }
                    }
                    break;
                }
                _ => {}
            }
            end += 1;
        }
    }
}

pub fn parse_mtl(obj_name: &str, mtl_path: &str) -> Vec<MtlItem> {
    let mut pb = path_of(obj_name);
    pb.pop();
    let mtl_p = Path::new(mtl_path);
    pb.push(mtl_p);
    let content = read_file(&pb).unwrap();

    let mut parser = MtlParser { items: Vec::new() };

    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    for c in content.chars() {
        match c {
            '\n' => {
                parser.parse_line(&content[line_start..line_end]);
                line_start = line_end + 1;
            }
            _ => {}
        }
        line_end += 1;
    }
    parser.items
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
