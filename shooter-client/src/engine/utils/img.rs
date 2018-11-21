use img::{save_buffer,ColorType};
use std::path::Path;

pub fn save_as_image(path: &Path, pixels: &[u8], width: u32, height: u32, bbp: u32) {
    let color_type = match bbp {
        4 => { ColorType::RGBA(8) },
        _ => panic!("save_as_image: Unsupported bbp: {}", bbp),
    };
    println!("Trying to save image at: {:#?}", path);
    match save_buffer(path, pixels, width, height, color_type) {
        Ok(_) => (),
        Err(_) => panic!("Error trying to save pixels as image"),
    }
    println!("Image was saved");
}

pub fn save_as_image_in_current_dir(file_name: &str, pixels: &[u8], width: u32, height: u32, bbp: u32) {
    let mut cdir = ::std::env::current_dir().unwrap();
    cdir.push(file_name);
    save_as_image(&cdir, pixels, width, height, bbp);
}
