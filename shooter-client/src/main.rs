extern crate shooter_common;
extern crate glfw;
extern crate gl;

mod shader;
mod mesh;
mod drawing;

use shader::*;
use mesh::*;
use drawing::*;

use glfw::{Action, Context, Key};

use gl::types::*;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(600,800, "Shooter", glfw::WindowMode::Windowed).expect("failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const GLvoid);


    let program = ShaderProgram::create_program("default");




    let mesh = Mesh::create_quad();

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
