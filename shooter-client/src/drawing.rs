use super::gl;
use super::gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use shader::*;
use std::collections::HashMap;
use super::na::Matrix4;

pub trait Drawable {
    fn draw(&self, dc: &DrawContext);
}

pub struct DrawContext {
    camera_matrix: Matrix4<f32>,
    shader_cache: HashMap<String, ShaderProgram>,
    vertex_array: VertexArray,
    pub width: u32,
    pub height: u32,
}

impl DrawContext {
    pub fn new(width: u32, height: u32, camera_matrix: Matrix4<f32>) -> DrawContext {

        let vertex_array = VertexArray::new();

        DrawContext {
            camera_matrix: camera_matrix,
            shader_cache: HashMap::new(),
            vertex_array: vertex_array,
            width: width,
            height: height,
        }
    }

    pub fn set_camera_matrix(&mut self, camera_matrix: Matrix4<f32>) {
        self.camera_matrix = camera_matrix;
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.camera_matrix
    }

    pub fn add_shader_program(&mut self, name: &str, shader: ShaderProgram) {
        self.shader_cache.insert(name.to_string(), shader);
    }

    pub fn use_shader_program(&self, name: &str) -> &ShaderProgram {
        let ret = self.shader_cache.get(name).unwrap();
        ret.use_program();
        ret
    }

    pub fn bind(&mut self) {
        self.vertex_array.bind();
    }

    pub fn unbind(&mut self) {
        self.vertex_array.unbind();
    }

    pub fn clear(&mut self, color: (f32,f32,f32,f32)) {
        unsafe {
            gl::ClearColor(color.0,color.1,color.2,color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

}


pub struct VertexArray {
    handle: GLuint
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        };
        VertexArray {
            handle: vao,
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}


pub enum GLDataType {
    Float,
}

pub struct VertexAttribute {
    n_components: u32,
    data_type: GLDataType,
    location: u32,
}

impl VertexAttribute {
    pub fn new(location: u32, data_type: GLDataType, n_components: u32) -> VertexAttribute {
        VertexAttribute {
            n_components: n_components,
            data_type: data_type,
            location: location,
        }
    }

    pub fn enable(&mut self, offset: u32, stride: u32) {
        unsafe {
            let (t, size) = match &self.data_type {
                &GLDataType::Float => { (gl::FLOAT, mem::size_of::<GLfloat>() as i32) }
            };
            gl::VertexAttribPointer(self.location, self.n_components as i32, t, gl::FALSE,
                                    (stride as i32 * size) as i32,
                                    (offset as i32 * size) as *const c_void);
            gl::EnableVertexAttribArray(self.location);
        }
    }
}

pub struct Batch {
    vertices: Vec<GLfloat>,
    indices: Vec<GLuint>,

    vbo: GLuint,
    ebo: GLuint,
}

impl Batch {
    pub fn new() -> Batch {
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        }

        let mut positions = VertexAttribute::new(0, GLDataType::Float, 3);
        positions.enable(0,5);

        let mut ux_coords = VertexAttribute::new(1, GLDataType::Float, 2);
        ux_coords.enable(3,5);

        Batch {
            vertices: Vec::new(),
            indices: Vec::new(),

            vbo: vbo,
            ebo: ebo,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn update_data(&mut self) {

        unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * self.vertices.len()) as isize,
                       mem::transmute(self.vertices.first().unwrap()), gl::STATIC_DRAW);


        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * self.indices.len()) as GLsizeiptr,
                       mem::transmute(self.indices.first().unwrap()), gl::STATIC_DRAW);
        }

    }
}

impl Drawable for Batch {
    fn draw(&self, dc: &DrawContext) {
        draw_elements(self.indices.len() as i32);
    }
}

fn draw_elements(n_elements: i32) {
    unsafe {
        gl::DrawElements(gl::TRIANGLES, n_elements as i32, gl::UNSIGNED_INT, ptr::null());
    }
}
