use img::{save_buffer,ColorType};
use std::path::Path;

pub fn save_as_image(path: &Path, pixels: &[u8], width: u32, height: u32, bbp: u32) {
    let color_type = match bbp {
        4 => { ColorType::RGBA(8) },
        _ => panic!("save_as_image: Unsupported bbp: {}", bbp),
    };
    save_buffer(path, pixels, width, height, color_type);
}
