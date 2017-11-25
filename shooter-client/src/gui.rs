use std;
use std::rc::Rc;
use na::{Vector2,Vector4};
use mesh::*;
use drawing::*;

trait Layout {
    fn measure(&self) -> Vector2<f64>;
    fn arrange(&self, children: Vec<Box<Layout>>);
}

enum VerticalAlignment {
    Stretch,
    Top,
    Center,
    Bottom
}

enum HorizontalAlignment {
    Stretch,
    Left,
    Center,
    Right
}

pub struct Panel {
    //layout: Rc<Layout>,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    width: f64,
    height: f64,
}

impl Panel {
    pub fn new(width: f64, height: f64) -> Panel {
        Panel {
            width: width,
            height: height,
            vertical_alignment: VerticalAlignment::Stretch,
            horizontal_alignment: HorizontalAlignment::Stretch,
        }
    }
}

pub struct TextElement {
    font_size: f32,
    value: String,
}

pub struct Shape {
    panel: Panel,
    color: Vector4<f64>,
}

impl Shape {
    pub fn new(color: Vector4<f64>, panel: Panel) -> Shape {
        Shape {
            panel: panel,
            color: color,
        }
    }
}


impl Drawable for Shape {
    fn draw(&self, dc: &DrawContext) {
        let mesh = Mesh::create_rect(self.panel.width as _, self.panel.height as _);

        let program_ref = dc.use_shader_program("solid_color");
        program_ref.set_float4("solid_color",
                               (self.color.x as _,
                                self.color.y as _,
                                self.color.z as _,
                                self.color.w as _));

        program_ref.set_float2("screen_size", (dc.width as _, dc.height as _));

        mesh.draw(dc);
    }
}
