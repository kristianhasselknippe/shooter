pub mod texture;

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

#[derive(Debug,Clone)]
struct BufferData {
    target: GLenum,
}

#[derive(Debug,Clone)]
pub enum BufferType {
    VertexArrayBuffer,
    ElementArrayBuffer,
}

#[derive(Debug,Clone)]
pub struct Buffer {
    buffer_type: BufferType,
    handle: BufferHandle,
    data: BufferData,
}

fn gl_print_error(_msg: &str) {
    //print!("{} - ", msg);
    //check_gl_errors();
}

fn gen_buffer() -> BufferHandle {
    unsafe {
        let mut out = 0;
        gl::GenBuffers(1, &mut out);
        gl_print_error("GenBuffers");
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
        },
    }
}

pub fn gen_element_array_buffer() -> Buffer {
    Buffer {
        buffer_type: BufferType::ElementArrayBuffer,
        handle: gen_buffer(),
        data: BufferData {
            target: gl::ELEMENT_ARRAY_BUFFER,
        },
    }
}

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl_print_error("ClearColor");
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl_print_error("Clear");
    }
}

impl Buffer {
    pub fn upload_data(&mut self, data: *const u8, len: isize) {
        println!(
            "Uploading data of len: {:?}, to target: {}",
            len, self.data.target
        );
        unsafe {
            gl::BufferData(
                self.data.target,
                len,
                data as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl_print_error("BufferData");
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(self.data.target, self.handle);
        }
        gl_print_error("BindBuffer");
    }

    pub fn unbind(&mut self) {
        unsafe { gl::BindBuffer(self.data.target, 0) }
        gl_print_error("UnbindBuffer");
    }
}

pub fn enable_vertex_attribs(attribs: &[VertexAttribute]) {
    let mut stride = 0;
    for a in attribs {
        stride += a.num_comps * GL_TYPE_TO_SIZE[&a.data_type]
    }

    println!("Stride: {}", stride);

    let mut offset = 0;
    for attrib in attribs {
        unsafe {
            println!(
                "VertexAttribPointer: {},{},{},{},{}",
                attrib.location, attrib.num_comps, attrib.data_type, stride, offset
            );
            gl::VertexAttribPointer(
                attrib.location,
                attrib.num_comps,
                attrib.data_type,
                gl::FALSE,
                stride, // Tightly packed atm
                offset as *const GLvoid,
            );
            gl_print_error("VertexAttribPointer");
            gl::EnableVertexAttribArray(attrib.location);
            gl_print_error("EnableVertexAttribArray");
        }
        offset += attrib.num_comps * GL_TYPE_TO_SIZE[&attrib.data_type]
    }
}

pub fn draw_triangles(num_indices: GLsizei, element_type: GLenum) {
    gl_print_error("Before draw triangles");
    unsafe { gl::DrawElements(gl::TRIANGLES, num_indices, element_type, 0 as _) }
    gl_print_error("DrawElements");
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
    pub handle: GLuint,
}

pub fn gen_vertex_array() -> VertexArray {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl_print_error(&format!("GenVertexArrays {}", vao));
        assert!(vao != 0);
        VertexArray { handle: vao }
    }
}

impl VertexArray {
    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.handle);
            gl_print_error(&format!("BindVertexArray {}", self.handle));
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl_print_error("UnbindVertexArray0");
        }
    }
}

pub fn check_gl_errors() {
    unsafe {
        let error = gl::GetError();
        match error {
            gl::NO_ERROR => {
                println!("No GL error");
            }
            gl::INVALID_ENUM => {
                println!("GL: Invalid enum error");
            }
            gl::INVALID_VALUE => {
                println!("GL: Invalid value");
            }
            gl::INVALID_OPERATION => {
                println!("GL: Invalid operation");
            }
            gl::INVALID_FRAMEBUFFER_OPERATION => {
                println!("GL: Invalid framebuffer operation");
            }
            gl::OUT_OF_MEMORY => {
                println!("GL: Out of memory");
            }
            _ => {
                println!("GL: Unknown error code");
            }
        }
    }
}

fn gl_get_string<'a>(name: GLenum) -> Result<&'a str, ::std::str::Utf8Error> {
    unsafe {
        let str = gl::GetString(name);
        gl_print_error("GetString");
        let cstr = ::std::ffi::CStr::from_ptr(str as *const i8);
        cstr.to_str()
    }
}

pub fn get_gl_version() -> String {
    if let Ok(ret) = gl_get_string(gl::VERSION) {
        ret.to_string()
    } else {
        panic!("Unable to get GL VERSION string");
    }
}

pub fn viewport(width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) };
}

