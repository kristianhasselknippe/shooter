use std;
use na::{Vector2,Vector4};

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

struct Panel<'a> {
    layout: &'a (Layout + 'a),
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    width: f64,
    height: f64,
}

struct Shape<'a> {
    panel: Panel<'a>,
    color: Vector4<f64>,
}
