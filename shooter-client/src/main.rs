extern crate shooter_common;
extern crate glutin;
extern crate libc;
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
mod input;

use shader::*;
use mesh::*;
use drawing::*;
use texture::*;
use entity::*;
use text::*;
use input::*;


use std::path::Path;

use gl::types::*;

fn main() {

    let window_size = (600,800);

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Shooter".to_string())
        .with_dimensions(window_size.0, window_size.1)
        .with_vsync()
        .build(&events_loop)
        .unwrap();


    unsafe {
        window.make_current()
    }.unwrap();

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0,1.0,0.0,1.0);
    }


    let draw_context = DrawContext::new(window_size.0, window_size.1);
    let text = Text::new("This is a string", &draw_context);


    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } => {
                    running = false;
                },
                _ => ()
            }
        });


        draw_context.bind();

        draw_context.clear((1.0,0.0,1.0,1.0));

        text.bind();
        draw_context.draw();

        draw_context.unbind();

        window.swap_buffers().unwrap();

    }







    let mut draw_context = DrawContext::new(window_size.0, window_size.1);

    let text = Text::new("this is some text", &draw_context);

    draw_context.bind();
    //mesh.bind();
    draw_context.unbind();



    /*while !window.should_close() {

        draw_context.bind();

        draw_context.clear((1.0,0.0,1.0,1.0));

        text.bind();
        draw_context.draw();

        draw_context.unbind();

        window.swap_buffers();


    }*/
}
