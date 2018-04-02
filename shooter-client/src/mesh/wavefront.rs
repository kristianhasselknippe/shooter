use utils::file::*;
use gl::types::*;
use super::{Vertex3,Normal,TexCoord};
use super::model::MemModel;

struct WavefrontModel {
    object: MemModel
}

fn parse_obj(line: &str) {
    println!("Parsed obj: {}", line);
}

fn split_parts(content: &str) -> Vec<&str> {
    content.split(|c| c == ' ' || c == ',').collect()
}

fn split_parts_f32(content: &str) -> Vec<f32> {
    split_parts(content).iter().map(|p| {
        let trimmed = p.trim();
        if let Ok(ret) = trimmed.parse::<f32>() {
            ret
        } else {
            panic!("Unable to parse float, {:?}", trimmed);
        }
    }).collect()
}

fn parse_vert(line: &str) {
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

fn parse_texcoord(line: &str) {
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

fn parse_face(line: &str) {
    
}

fn parse_s(line: &str) {
    
}

fn parse_usemtl(line: &str) {
    
}

fn parse_line(line: &str) {
    println!("Parsing line: {}", line);
    let mut end = 0;
    for c in line.chars() {
        match c {
            ' ' | ',' => {
                let subline = &line[0..end];
                end += 1;
                match subline {
                    "v" => {
                        parse_vert(&line[end..line.len() as usize]);
                    },
                    "vt" => {
                        parse_texcoord(&line[end..line.len() as usize]);
                    },
                    "f" => {
                        parse_face(&line[end..line.len() as usize]);
                    },
                    "s" => {
                        parse_s(&line[end..line.len() as usize]);
                    },
                    "o" => {
                        parse_obj(&line[end..line.len() as usize]);
                    },
                    "#" => {
                        println!("Comment {}", &line[end..line.len() as usize]);
                    },
                    "usemtl" => {
                        parse_usemtl(&line[end..line.len() as usize]);
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

pub fn parse_wavefront(content: &str) -> MemModel {
    let mut vertices: Vec<Vertex3> = Vec::new();
    let mut normals: Vec<Normal> = Vec::new();
    let mut indices: Vec<GLuint> = Vec::new();
    
    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    for c in content.chars() {
        match c {
            '\n' => {
                parse_line(&content[line_start..line_end]);
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
