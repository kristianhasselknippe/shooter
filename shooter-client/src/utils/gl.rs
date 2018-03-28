use std::collections::HashMap;
use gl;
use gl::types::*;
use std::mem::size_of;

lazy_static! {
    static ref GL_TYPE_TO_SIZE: HashMap<GLenum, GLsizei> = {
        hashmap! {
            gl::FLOAT => size_of::<GLfloat>() as GLsizei
        }
    };
}


type BufferHandle = GLuint;

struct BufferData {
    target: GLenum,
    is_bound: bool,
}

pub enum BufferType {
    VertexArrayBuffer,
    ElementArrayBuffer,
}

pub struct Buffer {
    buffer_type: BufferType,
    handle: BufferHandle,
    data: BufferData,
}

fn gen_buffer() -> BufferHandle {
    unsafe {
        let mut out = 0;
        gl::GenBuffers(1, &mut out);
        assert!(out != 0);
        out
    }
}

pub fn gen_vertex_array_buffer() -> Buffer {
    Buffer {
        buffer_type: BufferType::VertexArrayBuffer,
        handle: gen_buffer(),
        data: BufferData {
            target: gl::ARRAY_BUFFER,
            is_bound: false,
        },
    }
}

pub fn gen_element_array_buffer() -> Buffer {
    Buffer {
        buffer_type: BufferType::ElementArrayBuffer,
        handle: gen_buffer(),
        data: BufferData {
            target: gl::ELEMENT_ARRAY_BUFFER,
            is_bound: false,
        },
    }
}

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

impl Buffer {
    pub fn upload_data(&mut self, data: *const u8, len: isize) {
        assert!(self.data.is_bound,
                "Attempted to upload data to unbound buffer");
        println!("Uploading data of len: {:?}", len);
        unsafe {
            gl::BufferData(self.data.target,
                           len as isize,
                           data as *const GLvoid,
                           gl::STATIC_DRAW);
        }
    }

    pub fn bind(&mut self) {
        assert!(!self.data.is_bound,
                "Attempted to rebind an already bound vertex buffer");
        unsafe {
            gl::BindBuffer(self.data.target, self.handle);
        }
        self.data.is_bound = true;
    }

    pub fn unbind(&mut self) {
        assert!(self.data.is_bound,
                "Attempted to unbind an already unbound vertex buffer");
        unsafe { gl::BindBuffer(self.data.target, 0) }
        self.data.is_bound = false;
    }

    pub fn enable_vertex_attrib(&mut self, attribs: &[VertexAttribute]) {
        assert!(self.data.is_bound,
                "Attempted to enable vertex attribute on unbound vertex buffer");

        let mut stride = 0;
        for a in attribs {
            stride += a.num_comps * GL_TYPE_TO_SIZE[&a.data_type]
        }

        let mut offset = 0;
        for attrib in attribs {
            unsafe {
                // Enable the attribute array for location
                gl::EnableVertexAttribArray(attrib.location);
                // Define the shape of the data for this attribute
                gl::VertexAttribPointer(attrib.location,
                                        attrib.num_comps,
                                        attrib.data_type,
                                        gl::FALSE,
                                        stride,
                                        offset as *const GLvoid)

            }
            offset += attrib.num_comps * GL_TYPE_TO_SIZE[&attrib.data_type]
        }
    }
}

pub fn draw_triangles(num_indices: GLsizei, element_type: GLenum) {
    unsafe { gl::DrawElements(gl::TRIANGLES, num_indices, element_type, 0 as _) }
}

pub struct VertexAttribute {
    location: GLuint,
    data_type: GLenum,
    num_comps: GLsizei,
}

impl VertexAttribute {
    pub fn new(location: GLuint, data_type: GLenum, num_comps: GLsizei) -> VertexAttribute {
        VertexAttribute {
            location: location,
            data_type: data_type,
            num_comps: num_comps,
        }
    }
}

pub struct VertexArray {
    handle: GLuint,
    is_bound: bool,
}

pub fn gen_vertex_array() -> VertexArray {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert!(vao != 0);
        VertexArray {
            is_bound: false,
            handle: vao,
        }
    }
}

impl VertexArray {
    pub fn bind(&mut self) {
        assert!(!self.is_bound,
                "Attempted to rebind an already bound vertex array");
        unsafe {
            gl::BindVertexArray(self.handle);
        }
        self.is_bound = true;
    }

    pub fn unbind(&mut self) {
        assert!(self.is_bound, "Attempted to unbind an unbound vertex array");
        unsafe {
            gl::BindVertexArray(0);
        }
        self.is_bound = false;
    }
}
