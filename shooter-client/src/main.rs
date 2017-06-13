extern crate shooter_common;
extern crate glfw;
extern crate gl;

mod shader;
mod mesh;
mod drawing;

use shader::{create_vertex_shader,create_fragment_shader,create_program};
use mesh::*;
use drawing::*;

use glfw::{Action, Context, Key};

use gl::types::*;
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


    let vertices: Vec<GLfloat> = vec![
        0.5,  0.5, 0.0,  // Top Right
        0.5, -0.5, 0.0,  // Bottom Right
        -0.5, -0.5, 0.0,  // Bottom Left
        -0.5,  0.5, 0.0   // Top Left
    ];

    let indices: Vec<GLuint> = vec![  // Note that we start from 0!
        0, 1, 3,   // First Triangle
        1, 2, 3    // Second Triangle
    ];

    let mesh = Mesh::new(vertices, indices);

    program.use_program();

    let draw_context = DrawContext::new();
    draw_context.bind();
    mesh.bind();
    draw_context.unbind();


    while !window.should_close() {

        program.use_program();
        draw_context.bind();
        {
            draw_context.draw();
        }
        draw_context.unbind();

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
