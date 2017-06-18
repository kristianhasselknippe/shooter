use rusttype::{FontCollection, Scale, point, PositionedGlyph};
use std::io::Write;

use super::gl;
use super::gl::types::*;
use super::drawing::*;
use super::mesh::*;
use super::texture::*;
use super::shader::*;

pub struct Glyph {
    data: Vec<u8>,
    width: u32,
    height: u32,
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
}


impl Font {
    pub fn new() -> Font {
        let font_data = include_bytes!("../assets/fonts/emulogic.ttf");
        let collection = FontCollection::from_bytes(font_data as &[u8]);
        let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

        // Desired font pixel height
        let height: f32 = 12.4; // to get 80 chars across (fits most terminals); adjust as desired
        let pixel_height = height.ceil() as usize;

        // 2x scale in x direction to counter the aspect ratio of monospace characters.
        let scale = Scale { x: height*2.0, y: height };

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let positioned_glyphs: Vec<PositionedGlyph> = font.layout("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", scale, offset).collect();

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
                glyph_data[(x + (w as u32*y)) as usize] = (v * 255.0) as u8;
            });

            glyphs.push(Glyph::new(glyph_data, w as u32, h as u32));
        }

        Font {
            glyphs: glyphs
        }
    }

    pub fn total_size(&self) -> (u32, u32) {
        let mut tot_width = 0;
        let mut height = 0;
        for g in &self.glyphs {
            tot_width += g.width;
            if g.height > height {
                height = g.height;
            }
        }
        (tot_width, height)
    }

    pub fn data(&self) -> Vec<u8> {
        let tot_size = self.total_size();
        let n_pixels = tot_size.0 * tot_size.1;
        let mut ret = Vec::with_capacity(n_pixels as usize);
        for i in 0..n_pixels {
            ret.push(0);
        }
        let mut x_offset = 0;
        for g in &self.glyphs {
            let w = g.width;
            let h = g.height;
            let mut x = 0;
            let mut y = 0;
            for p in &g.data {
                if x == g.width {
                    x = 0;
                    y += 1;
                }
                todo -- need to fill in the texture correctly in the x direction
                ret[(x + x_offset + (y * tot_size.0)) as usize] = *p;
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

        println!("FontSize: {:?}", font_size);

        let font_width = (font_size.0 as f32/dc.width as f32);
        let font_height = (font_size.1 as f32/dc.height as f32);

        println!("W: {}, H: {}", font_width, font_height);
        let mesh = Mesh::create_rect(font_width, font_height);

        let font_data = font.data();

        let shader = ShaderProgram::create_program("text");

        let texture = Texture::from_data_u8((font_size.0 as i32, font_size.1 as i32), font_data);

        Text {
            val: val.to_string(),

            font: font,
            mesh: mesh,
            texture: texture,
            shader: shader,
        }
    }

    pub fn bind(&self) {
        self.mesh.bind();
        self.texture.bind();
        self.shader.use_program();
    }
}