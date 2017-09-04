use rusttype::{FontCollection, Scale, point, PositionedGlyph};
use std::io::Write;
use std::path::Path;

use super::gl;
use super::gl::types::*;
use super::drawing::*;
use super::mesh::*;
use super::texture::*;
use super::shader::*;

use super::image::ColorType;

pub struct Glyph {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Glyph {
    pub fn new(data: Vec<u8>, w: u32, h: u32) -> Glyph {
        Glyph {
            data: data,
            width: w,
            height: h,
        }
    }
}

pub struct Font {
    glyphs: Vec<Glyph>,
    cell_size: u32,
}


impl Font {
    pub fn new() -> Font {
        let font_data = include_bytes!("../assets/fonts/Lato-Regular.ttf");
        let collection = FontCollection::from_bytes(font_data as &[u8]);
        let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

        // Desired font pixel height
        let height: f32 = 14.0; // to get 80 chars across (fits most terminals); adjust as desired
        let pixel_height = height.ceil() as usize;

        // 2x scale in x direction to counter the aspect ratio of monospace characters.
        let scale = Scale { x: height, y: height };

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let positioned_glyphs: Vec<PositionedGlyph> = font.layout("ABCDEFGHIJKLMNOPQRSTUVWXYZ", scale, offset).collect();

        let mut glyphs = Vec::new();

        for positioned_glyph in positioned_glyphs {
            let bounding_box = positioned_glyph.pixel_bounding_box().unwrap();
            let w = bounding_box.width();
            let h = bounding_box.height();

            let mut glyph_data: Vec<u8> = Vec::with_capacity((w*h) as usize);
            for i in 0..(w*h) {
                glyph_data.push(0);
            }

            positioned_glyph.draw(|x,y,v| {
                //println!("{} - {} - V: {}", x,y,v);
                glyph_data[(x + (w as u32*y)) as usize] = (v * 255.0) as u8;
            });

            glyphs.push(Glyph::new(glyph_data, w as u32, h as u32));
        }

        let mut cell_size = 0;
        for g in &glyphs {
            //println!("W: {}, H:{}", g.width, g.height);
            if g.width > cell_size {
                cell_size = g.width;
            }
            if g.height > cell_size {
                cell_size = g.height;
            }

            /*for y in 0..g.height {
                for x in 0..g.width {
                    /*if (g.data[(y * g.width + x) as usize] > 0) {
                        print!("X");
                    } else {
                        print!(" ");
                    }*/
                }
                //print!("\n");
            }*/
            //println!("=============; W:{}, H:{}", g.width, g.height);
        }

        println!("CellSize: {}", cell_size);

        Font {
            glyphs: glyphs,
            cell_size: cell_size, //TODO: let the user control this? Or just get rid of it at some point
        }
    }

    pub fn total_size(&self) -> (u32, u32) {
        /*let mut tot_width = 0;
        let mut height = 0;
        for g in &self.glyphs {
            tot_width += g.width;
            if g.height > height {
                height = g.height;
            }
        }*/
        (self.cell_size * self.glyphs.len() as u32, self.cell_size)
    }

    pub fn data(&self) -> Vec<u8> {
        let tot_size = self.total_size();
        println!("total size: {:?}", tot_size);

        let n_glyphs = self.glyphs.len();
        let cell_size = self.cell_size;
        let width = n_glyphs * cell_size as usize;
        let n_pixels = self.glyphs.len() * (cell_size * cell_size) as usize;

        let mut ret = Vec::with_capacity(n_pixels as usize);
        for i in 0..n_pixels {
            ret.push(0);
        }

        let mut x_offset = 0;
        for g in &self.glyphs {
            for y in 0..g.height {
                for x in 0..g.width {
                    let index = (x + (y * g.width)) as usize;
                    let d = g.data[index];
                    ret[(x_offset + (x + (y*width as u32))) as usize] = d;
                }
            }
            x_offset += cell_size as u32;
        }


        /*for x in 0..width {
            for y in (0..cell_size).rev() {
                let foo = ret[(x + (y*width)) as usize];
                if (foo > 0) {
                    print!("XX");
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }*/
        ret
    }

    pub fn rgba_data(&self) -> Vec<u8> {
        let data = self.data();

        let mut ret = Vec::with_capacity(data.len() * 4);
        for d in data {
            for i in 0..4 {
                ret.push(d);
            }
        }
        ret
    }
}

pub struct Text {
    val: String,

    shader: ShaderProgram,
    font: Font,
    mesh: Mesh,
    texture: Texture,
}

impl Text {
    pub fn new(val: &str, dc: &DrawContext) -> Text {
        let font = Font::new();

        let font_size = font.total_size();

        //println!("FontSize: {:?}", font_size);

        let font_width = (font_size.0 as f32/dc.width as f32);
        let font_height = (font_size.1 as f32/dc.height as f32);
        //println!("W: {}, H: {}", font_width, font_height);

        let mesh = Mesh::create_rect(font_width, font_height);

        let font_data = font.rgba_data();

        Image::save_png(Path::new("bitmap_font_text.png"), font_data.as_slice(), font_size.0 as u32, font_size.1 as u32, ColorType::Gray(8));

        let shader = ShaderProgram::create_program("text");

        let texture = Texture::from_data_u8((font_size.0 as i32, font_size.1 as i32), &font_data, &ImageFormat::RGBA);

        Text {
            val: val.to_string(),

            font: font,
            mesh: mesh,
            texture: texture,
            shader: shader,
        }
    }
}


impl Drawable for Text {
    fn draw(&self) {
        self.texture.bind(TextureUnit::Unit0);
        self.shader.use_program();

        self.mesh.draw_now();
    }
}
