use img::{load, ImageFormat, DynamicImage, GenericImage};
use std::io::BufReader;
use std::path::Path;
use std::fs::File;

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn load_texture(asset_name: &Path) -> Image {
    let reader = BufReader::new(File::open(asset_name).unwrap());
    match load(reader, ImageFormat::PNG) {
        Ok(dyn_img) => {
            Image {
                data: dyn_img.raw_pixels(),
                width: dyn_img.dimensions().0,
                height: dyn_img.dimensions().1,
            }
            /*match dyn_img {
                DynamicImage::ImageLuma8(gray_img) => {
                    gray_img.raw_pixels()
                },
                DynamicImage::ImageLumaA8(gray_alpha_img) => {
                    gray_alpha_img.raw_pixels()
                },
                DynamicImage::ImageRgb8(rgb_img) => {
                    rgb_img.raw_pixels()
                },
                DynamicImage::ImageRgba8(rgba_img) => {
                    rgba_img.raw_pixels()
                },
            }*/
        },
        Err(e) => {
            panic!("Could not load image: {:?}", e);
        }
    }
}
