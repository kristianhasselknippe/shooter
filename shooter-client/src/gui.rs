use na::{Vector2,Vector4};
use mesh::mesh::*;
use drawing::*;

trait Layout {
    fn measure(&self, gui_data: &mut GuiElementData) -> Vector2<f64>;
    
    fn arrange(&self, gui_data: &mut GuiElementData);
}

struct DefaultLayout {}
/*impl Layout for DefaultLayout {
    fn measure(&self, gui_data: &mut GuiElementData) -> Vector2<f64> {
        
    }
    
    fn arrange(&self, gui_data: &mut GuiElementData) {
        
    }
}*/

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

pub struct GuiElementData {
    pos: Option<Vector2<f64>>,
    size: Option<Vector2<f64>>,
    margin: Option<Vector4<f64>>,
}

impl GuiElementData {
    pub fn identity() -> GuiElementData {
        GuiElementData {
            pos: None,
            size: None,
            margin: None,
        }
    }

    pub fn new(pos: Vector2<f64>, size: Vector2<f64>) -> GuiElementData {
        GuiElementData {
            pos: Some(pos),
            size: Some(size),
            margin: None,
        }
    }

    pub fn with_margin(&mut self, margin: Vector4<f64>) -> &mut Self {
        self.margin = Some(margin);
        self
    }
}

pub struct Panel {
    //layout: Rc<Layout>,
    
    children: Vec<Panel>,
    drawables: Vec<Box<Drawable>>,

    gui_data: GuiElementData,
    
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
}


impl Panel {
    pub fn new(pos: Vector2<f64>, size: Vector2<f64>) -> Panel {
        Panel {
            children: Vec::new(),
            drawables: Vec::new(),

            gui_data: GuiElementData::new(pos, size),
            
            vertical_alignment: VerticalAlignment::Stretch,
            horizontal_alignment: HorizontalAlignment::Stretch,
        }
    }

    pub fn add_drawable(&mut self, d: Box<Drawable>) {
        self.drawables.push(d);
    }
}

impl Drawable for Panel {
    fn draw(&self, dc: &DrawContext) {
        for d in &self.drawables {
            d.draw(dc);
        }
        for c in &self.children {
            c.draw(dc);
        }
    }
}

struct TextElement {
    gui_data: GuiElementData,
    font_size: f32,
    value: String,
}

pub struct Shape {
    gui_data: GuiElementData,
    color: Vector4<f64>,
}

impl Shape {
    pub fn new(color: Vector4<f64>, pos: Vector2<f64>, size: Vector2<f64>) -> Shape {
        Shape {
            gui_data: GuiElementData::new(pos, size),
            color: color,
        }
    }

    pub fn new_with_data(color: Vector4<f64>, data: GuiElementData) -> Shape {
        Shape {
            gui_data: data,
            color: color,
        }
    }

    pub fn new_with_color(color: Vector4<f64>) -> Shape {
        Shape {
            gui_data: GuiElementData::identity(),
            color: color,
        }
    }
}

impl Drawable for Shape {
    fn draw(&self, dc: &DrawContext) {
        if let (Some(pos), Some(size)) = (self.gui_data.pos, self.gui_data.size) {
            let mesh = GlMesh::create_from_pos_size(pos, size);
            let program_ref = dc.use_shader_program("solid_color");
            program_ref.set_float4("solid_color",
                                   (self.color.x as _,
                                    self.color.y as _,
                                    self.color.z as _,
                                    self.color.w as _));

            program_ref.set_float2("screen_size", (dc.width as _, dc.height as _));
            program_ref.set_float2("shape_size", (size.x as _, size.y as _));

            mesh.draw(dc);
        }

        
    }
}
