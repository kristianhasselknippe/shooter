use img::{load, GenericImageView, ImageFormat};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn load_texture(asset_name: &Path) -> Image {
    let reader = BufReader::new(File::open(asset_name).unwrap());
    match load(reader, ImageFormat::Png) {
        Ok(dyn_img) => Image {
            data: dyn_img.to_bytes(),
            width: dyn_img.dimensions().0,
            height: dyn_img.dimensions().1,
        },
        Err(e) => {
            panic!("Could not load image: {:?}", e);
        }
    }
}
