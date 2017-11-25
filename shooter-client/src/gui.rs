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
    Top,
    Center,
    Bottom
}

enum HorizontalAlignment {
    Left,
    Center,
    Right
}

struct Panel {
    layout: Rc<Layout>,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    width: f64,
    height: f64,
}

struct Text {
    font_size: f32,
    value: String,
}

struct Shape {
    panel: Panel,
    color: Vector4<f64>,
}


impl Drawable for Shape {
    fn draw(&self, dc: &DrawContext) {
        let mesh = Mesh::create_rect(self.panel.width as _, self.panel.height as _);
        mesh.draw_now();
    }
}
