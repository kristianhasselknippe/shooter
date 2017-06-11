extern crate shooter_common;

extern crate glfw;
use glfw::{Action, Context, Key};

extern crate gl;
use gl::types::*;
use std::mem;
use std::ffi::CString;
use std::ptr;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(600,800, "Shooter", glfw::WindowMode::Windowed).expect("failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const GLvoid);

    let vs = include_str!("default.vs");
    let fs = include_str!("default.fs");


    println!("VS: {}", vs);
    println!("FS: {}", fs);

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
            panic!("{}", std::str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
        vertex_shader
    };

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
            panic!("{}", std::str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
        fragment_shader
    };

    let program = unsafe {

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);

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
            panic!("{}", std::str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
        }

        program
    };

    let mut ebo = 0;
    let mut vbo = 0;
    let mut vao = 0;
    unsafe {

        let vertices: [GLfloat;12] = [
            0.5,  0.5, 0.0,  // Top Right
            0.5, -0.5, 0.0,  // Bottom Right
            -0.5, -0.5, 0.0,  // Bottom Left
            -0.5,  0.5, 0.0   // Top Left
        ];

        let indices: [GLuint;6] = [  // Note that we start from 0!
            0, 1, 3,   // First Triangle
            1, 2, 3    // Second Triangle
        ];

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::GenBuffers(1, &mut ebo);

        //glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as isize, mem::transmute(&vertices), gl::STATIC_DRAW);
        gl::UseProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertices[0]), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0 ,3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<GLuint>() * indices.len()) as GLsizeiptr,
                           mem::transmute(&indices[0]), gl::STATIC_DRAW);
        }
        gl::BindVertexArray(0); //unbind vao


    };


    while !window.should_close() {

        unsafe {
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }
            gl::BindVertexArray(0);
        };

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => (),
            }
        }
    }


}
