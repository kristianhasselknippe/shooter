pub mod texture;

use gl::types::*;
use std::collections::HashMap;
use std::mem::size_of;

lazy_static! {
    static ref GL_TYPE_TO_SIZE: HashMap<GLenum, GLsizei> = {
        hashmap! {
            gl::FLOAT => size_of::<GLfloat>() as GLsizei,
            gl::UNSIGNED_INT => size_of::<GLuint>() as GLsizei,
            gl::UNSIGNED_BYTE => size_of::<GLubyte>() as GLsizei,
        }
    };
}

type BufferHandle = GLuint;

#[derive(Debug, Clone)]
struct BufferData {
    target: GLenum,
}

#[derive(Debug, Clone)]
pub enum BufferType {
    VertexArrayBuffer,
    ElementArrayBuffer,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer_type: BufferType,
    handle: BufferHandle,
    data: BufferData,
}

fn gl_print_error(_msg: &str) {
    //print!("{} - ", _msg);
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

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl_print_error("ClearColor");
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl_print_error("Clear");
    }
}

impl Buffer {
    pub fn gen_vbo() -> Buffer {
        Buffer {
            buffer_type: BufferType::VertexArrayBuffer,
            handle: gen_buffer(),
            data: BufferData {
                target: gl::ARRAY_BUFFER,
            },
        }
    }

    pub fn gen_ebo() -> Buffer {
        Buffer {
            buffer_type: BufferType::ElementArrayBuffer,
            handle: gen_buffer(),
            data: BufferData {
                target: gl::ELEMENT_ARRAY_BUFFER,
            },
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle as *mut _);
        }
    }

    pub fn upload_data(&mut self, data: *const u8, len: isize) {
        /*println!(
            "Uploading data of len: {:?}, to target: {}",
            len, self.data.target
        );*/
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

pub struct VertexSpec {
    attributes: Vec<VertexAttribute>,
}

impl VertexSpec {
    pub fn new(attribs: Vec<VertexAttribute>) -> VertexSpec {
        VertexSpec {
            attributes: attribs,
        }
    }

    pub fn enable(&mut self) {
        let mut stride = 0;
        for a in &self.attributes {
            stride += a.num_comps * GL_TYPE_TO_SIZE[&a.data_type]
        }

        let mut offset = 0;
        for attrib in &self.attributes {
            unsafe {
                gl::VertexAttribPointer(
                    attrib.location,
                    attrib.num_comps,
                    attrib.data_type,
                    if attrib.norm { gl::TRUE } else { gl::FALSE },
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
    norm: bool,
}

impl VertexAttribute {
    pub fn new(
        location: GLuint,
        data_type: GLenum,
        num_comps: GLsizei,
        normalize: bool,
    ) -> VertexAttribute {
        VertexAttribute {
            location: location,
            data_type: data_type,
            num_comps: num_comps,
            norm: normalize,
        }
    }
}

pub struct VertexArray {
    pub handle: GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl_print_error(&format!("GenVertexArrays {}", vao));
            assert!(vao != 0);
            VertexArray { handle: vao }
        }
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.handle);
            gl_print_error(&format!("BindVertexArray {}", self.handle));
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.handle as *mut _);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl_print_error("UnbindVertexArray0");
        }
    }
}

/*impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.handle as *const _); }
    }
}*/

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
    println!("Setting viewport to 0,0 - {},{}", width, height);
    unsafe { gl::Viewport(0, 0, width, height) };
}

pub enum CullFace {
    Front,
    Back,
    FrontAndBack,
}

pub fn set_cull_face(cull_face: CullFace) {
    unsafe {
        match cull_face {
            CullFace::Front => gl::CullFace(gl::FRONT),
            CullFace::Back => gl::CullFace(gl::BACK),
            CullFace::FrontAndBack => gl::CullFace(gl::FRONT_AND_BACK),
        }
    }
}

pub enum Capability {
    /** If enabled, blend the computed fragment color values with the values in the color buffers. See glBlendFunc. */
    Blend,
    /** If enabled, cull polygons based on their winding in window coordinates. See glCullFace.*/
    CullFace,
    /** If enabled, do depth comparisons and update the depth buffer. Note that even if the depth buffer exists and the depth mask is non-zero, the depth buffer is not updated if the depth test is disabled. See glDepthFunc and glDepthRangef. */
    DepthTest,
    /** If enabled, dither color components or indices before they are written to the color buffer. */
    Dither,
    /** If enabled, an offset is added to depth values of a polygon's fragments produced by rasterization. See glPolygonOffset. */
    PolygonOffsetFill,
    /** If enabled, compute a temporary coverage value where each bit is determined by the alpha value at the corresponding sample location. The temporary coverage value is then ANDed with the fragment coverage value. */
    SampleAlphaToCoverage,
    /** If enabled, the fragment's coverage is ANDed with the temporary coverage value. If GL_SAMPLE_COVERAGE_INVERT is set to GL_TRUE, invert the coverage value. See glSampleCoverage. */
    SampleCoverage,
    /** If enabled, discard fragments that are outside the scissor rectangle. See glScissor. */
    ScissorTest,
    /** If enabled, do stencil testing and update the stencil buffer. See glStencilFunc and glStencilOp. */
    StencilTest,
}

pub fn enable(cap: Capability) {
    unsafe {
        match cap {
            Capability::Blend => gl::Enable(gl::BLEND),
            Capability::CullFace => gl::Enable(gl::CULL_FACE),
            Capability::DepthTest => gl::Enable(gl::DEPTH_TEST),
            Capability::Dither => gl::Enable(gl::DITHER),
            Capability::PolygonOffsetFill => gl::Enable(gl::POLYGON_OFFSET_FILL),
            Capability::SampleAlphaToCoverage => gl::Enable(gl::SAMPLE_ALPHA_TO_COVERAGE),
            Capability::SampleCoverage => gl::Enable(gl::SAMPLE_COVERAGE),
            Capability::ScissorTest => gl::Enable(gl::SCISSOR_TEST),
            Capability::StencilTest => gl::Enable(gl::STENCIL_TEST),
        }
    }
}

pub fn disable(cap: Capability) {
    unsafe {
        match cap {
            Capability::Blend => gl::Disable(gl::BLEND),
            Capability::CullFace => gl::Disable(gl::CULL_FACE),
            Capability::DepthTest => gl::Disable(gl::DEPTH_TEST),
            Capability::Dither => gl::Disable(gl::DITHER),
            Capability::PolygonOffsetFill => gl::Disable(gl::POLYGON_OFFSET_FILL),
            Capability::SampleAlphaToCoverage => gl::Disable(gl::SAMPLE_ALPHA_TO_COVERAGE),
            Capability::SampleCoverage => gl::Disable(gl::SAMPLE_COVERAGE),
            Capability::ScissorTest => gl::Disable(gl::SCISSOR_TEST),
            Capability::StencilTest => gl::Disable(gl::STENCIL_TEST),
        }
    }
}
