extern crate shooter_common;
extern crate glfw;
extern crate gl;
extern crate nalgebra as na;
extern crate image;
extern crate rusttype;

mod shader;
mod mesh;
mod drawing;
mod transform;
mod entity;
mod texture;
mod text;

use shader::*;
use mesh::*;
use drawing::*;
use texture::*;
use entity::*;
use text::*;


use std::path::Path;

use glfw::{Action, Context, Key};
use gl::types::*;

fn main() {

    let window_size = (600,800);

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(window_size.0,window_size.1, "Shooter", glfw::WindowMode::Windowed).expect("failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const GLvoid);

    let mut draw_context = DrawContext::new(window_size.0, window_size.1);

    let text = Text::new("this is some text", &draw_context);

    draw_context.bind();
    //mesh.bind();
    draw_context.unbind();



    while !window.should_close() {

        draw_context.bind();

        draw_context.clear((1.0,0.0,1.0,1.0));

        text.bind();
        draw_context.draw();

        draw_context.unbind();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                glfw::WindowEvent::Size(w,h) => {
                    println!("Resized: {},{}", w,h);
                    draw_context.width = w as u32;
                    draw_context.height = h as u32;
                }
                _ => (),
            }
        }
    }
}
