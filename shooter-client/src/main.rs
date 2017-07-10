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

use glutin::{ Event, WindowEvent, EventsLoop, WindowBuilder, DeviceEvent, ContextBuilder, GlWindow, GlContext };


use std::path::Path;

use gl::types::*;

fn main() {

    let window_size = (600,800);

    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_title("Shooter")
        .with_dimensions(window_size.0, window_size.1);

    let context = ContextBuilder::new()
        .with_vsync(true);

    let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

    println!("Foobar");


    unsafe {
        gl_window.make_current().unwrap();
    };

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0,1.0,0.0,1.0);
    }


    let draw_context = DrawContext::new(window_size.0, window_size.1);
    let text = Text::new("This is a string", &draw_context);


    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                    running = false;
                },
                Event::DeviceEvent { device_id, event } => {
                    match event {
                        DeviceEvent::Added => {
                        },
                        DeviceEvent::Removed => {
                        },
                        DeviceEvent::Motion { axis, value } => {
                            println!("Motion : {:?}", value);
                        },
                        DeviceEvent::Button { button, state } => {

                        },
                        DeviceEvent::Key(keyboard_input) => {

                        },
                        DeviceEvent::Text { codepoint } => {

                        },
                    }

                }
                _ => ()
            }
        });


        draw_context.bind();

        draw_context.clear((1.0,0.0,1.0,1.0));

        text.bind();
        draw_context.draw();

        draw_context.unbind();

        gl_window.swap_buffers().unwrap();

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
