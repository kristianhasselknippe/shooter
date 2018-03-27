use std::path::Path;
use std::ffi::CString;
use super::gl;
use super::gl::types::*;
use std::ptr;
use std::str;
use utils::file::read_file;

use na::Matrix4;

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

impl Shader {
    pub fn create_vertex_shader(vs: &str) -> Shader {
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
                for _ in 0..len { buf.push(0); }
                gl::GetShaderInfoLog(vertex_shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
            }
            vertex_shader
        };
        Shader {
            handle: vertex_shader
        }
    }


    pub fn create_fragment_shader(fs: &str) -> Shader {
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
                let error = str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8");
                println!("Error: {}", error);
                panic!("{}", error);

            }
            fragment_shader
        };
        Shader {
            handle: fragment_shader
        }
    }

    pub fn create_fragment_shader_from_path(path: &Path) -> Shader {
        let fs = read_file(path).unwrap();
        Shader::create_fragment_shader(&fs)
    }

    pub fn create_vertex_shader_from_path(path: &Path) -> Shader {
        let vs = read_file(path).unwrap();
        Shader::create_vertex_shader(&vs)
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

impl ShaderProgram {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn new(vs: &Shader, fs: &Shader) -> ShaderProgram {
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

    pub fn from_fragments(vs: &str, fs: &str) -> ShaderProgram {
        let vs = format!("#version 330 core\n
                          layout (location = 0) in vec3 position;\n
                          layout (location = 1) in vec2 tex_coord;\n
                          out vec2 TexCoord;\n
                          void main()\n
                          {{\n
                          {}\n
                          }}", vs);

        let fs = format!("#version 330 core\n
                          uniform sampler2D tex0;\n
                          in vec2 TexCoord;\n
                          out vec4 color;\n
                          void main()\n
                          {{\n
                          {}\n
                          }}", fs);

        //println!("vs: {}", vs);
        //println!("fs: {}", fs);

        let vertex_shader = Shader::create_vertex_shader(&vs);
        let fragment_shader = Shader::create_fragment_shader(&fs);
        ShaderProgram::new(&vertex_shader, &fragment_shader)
    }

    pub fn create_program(name: &str) -> ShaderProgram {
        let vertex_shader = Shader::create_vertex_shader_from_path(Path::new(&format!("src/shaders/{}.vs",name)));
        let fragment_shader = Shader::create_fragment_shader_from_path(Path::new(&format!("src/shaders/{}.fs",name)));
        ShaderProgram::new(&vertex_shader, &fragment_shader)
    }

    pub fn create_program_from_vert_frag(vert: &str, frag: &str) -> ShaderProgram {
        let vertex_shader = Shader::create_vertex_shader_from_path(Path::new(&format!("src/shaders/{}.vs",vert)));
        let fragment_shader = Shader::create_fragment_shader_from_path(Path::new(&format!("src/shaders/{}.fs",frag)));
        ShaderProgram::new(&vertex_shader, &fragment_shader)
    }

    pub fn set_bool(&self, name: &str, val: bool) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val as i32);
        }
    }

    pub fn set_int(&self, name: &str, val: i32) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val);
        }
    }

    pub fn set_float(&self, name: &str, val: f32) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val);
        }
    }
    
    pub fn set_float2(&self, name: &str, val: (f32,f32)) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val.0, val.1);
        }
    }

    pub fn set_float3(&self, name: &str, val: (f32,f32,f32)) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val.0, val.1, val.2);
        }
    }

    pub fn set_float4(&self, name: &str, val: (f32,f32,f32,f32)) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.handle, c_name.as_ptr()), val.0, val.1, val.2, val.3);
        }
    }

    pub fn set_mat4(&self, name: &str, val: Matrix4<f32>) {
        unsafe {
            let c_name = CString::new(name.as_bytes()).unwrap();
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.handle, c_name.as_ptr()), 1, gl::FALSE, val.as_slice().as_ptr());
        }
    }
}
