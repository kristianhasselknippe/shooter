use std::path::Path;
use std::fs::File;
use std::ffi::CString;
use super::gl;
use super::gl::types::*;
use std::ptr;
use std::str;
use std::io::Read;

pub struct Shader {
    handle: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}

pub struct ShaderProgram {
    pub handle: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

fn read_file(path: &Path) -> String {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    let bytes_read = f.read_to_string(&mut s).unwrap();
    if bytes_read == 0 {
        println!("Read 0 bytes from file. This is probably not right");
    }
    s
}

pub fn create_vertex_shader(path: &Path) -> Shader {
    let vs = read_file(path);
    let vertex_shader = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

        let c_str_vs = CString::new(vs.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vs.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity((len as usize) - 1);
            gl::GetShaderInfoLog(vertex_shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
        vertex_shader
    };
    Shader {
        handle: vertex_shader
    }
}


pub fn create_fragment_shader(path: &Path) -> Shader {
    let fs = read_file(path);
    let fragment_shader = unsafe {
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        let c_str_fs = CString::new(fs.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_fs.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity((len as usize) - 1);
            gl::GetShaderInfoLog(fragment_shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
        fragment_shader
    };
    Shader {
        handle: fragment_shader
    }
}

pub fn create_program(vs: &Shader, fs: &Shader) -> ShaderProgram {
    let program = unsafe {

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs.handle);
        gl::AttachShader(program, fs.handle);

        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
        }

        program
    };
    ShaderProgram {
        handle: program
    }
}
