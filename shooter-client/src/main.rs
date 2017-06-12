extern crate shooter_common;
extern crate glfw;
extern crate gl;

mod shader;
mod mesh;
mod drawing;

use shader::{create_vertex_shader,create_fragment_shader,create_program};

use glfw::{Action, Context, Key};

use gl::types::*;
use std::mem;
use std::ptr;
use std::path::Path;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(600,800, "Shooter", glfw::WindowMode::Windowed).expect("failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const GLvoid);

    let vertex_shader = create_vertex_shader(Path::new("src/default.vs"));
    let fragment_shader = create_fragment_shader(Path::new("src/default.fs"));
    let program = create_program(&vertex_shader, &fragment_shader);

    let mut ebo = 0;
    let mut vbo = 0;
    let mut vao = 0;

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

    unsafe {



        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::GenBuffers(1, &mut ebo);

        //glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as isize, mem::transmute(&vertices), gl::STATIC_DRAW);
        gl::UseProgram(program.handle);

        //gl::DeleteShader(vertex_shader);
        //gl::DeleteShader(fragment_shader);

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
            gl::UseProgram(program.handle);
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
