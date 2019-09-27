#![allow(unused_imports)]
extern crate gl;
extern crate glutin;
extern crate image as img;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate imgui_sys as imgui;
extern crate itertools;
extern crate nalgebra_glm as glm;
extern crate ncollide3d as nc;
extern crate num_traits;
extern crate ordered_float as of;
extern crate rusttype;
extern crate specs;
extern crate specs_derive;
extern crate time as t;

pub mod camera;
pub mod drawing;
pub mod fps_counter;
pub mod image;
pub mod input;
pub mod mesh;
pub mod shader;
pub mod time;
pub mod transform;
pub mod utils;
pub mod window;

use camera::*;
use drawing::*;
use glm::*;
use glutin::dpi::*;
use input::*;
use shader::*;
use specs::prelude::*;
use time::*;
use utils::gl::*;
use window::init_gl_window;

pub fn start_event_loop<F: Fn(f64, &Input, &mut Camera<f64>)>(perform_frame: &mut F) {
    let mut window_size = (800, 600);
    let (mut events_loop, gl_context) = init_gl_window(window_size);

    let window = gl_context.window();

    let dpi_factor = window.get_hidpi_factor();
    viewport(
        (window_size.0 as f32 * dpi_factor as f32) as i32,
        (window_size.1 as f32 * dpi_factor as f32) as i32,
    );

    let mut time = Time::new();
    let mut input = Input::new();
    let mut camera = Camera::new_perspective(
        16.0 / 9.0,
        3.14 / 4.0,
        1.0,
        1000.0,
        vec3::<f64>(0.0, 0.0, 8.0),
    );

    'running: loop {
        input.reset_mouse_delta();
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Resized(LogicalSize {
                        width: w,
                        height: h,
                    }) => {
                        println!("New Window size: {},{} - dpi: {}", w, h, dpi_factor);
                        window_size = (w as i32, h as i32);

                        let width = window_size.0 as f32 * dpi_factor as f32;
                        let height = window_size.1 as f32 * dpi_factor as f32;

                        //window.set_inner_size(LogicalSize::new(width as f64, height as f64));
                        viewport(width as i32, height as i32);
                        //gui.set_display_size((width, height));
                        camera.set_aspect(width as f64 / height as f64);
                    }
                    glutin::WindowEvent::KeyboardInput { input: i, .. } => {
                        input.update_glutin_keyboard_input(&i);
                    }
                    glutin::WindowEvent::MouseInput {
                        state: s,
                        button: mb,
                        modifiers: m,
                        ..
                    } => {
                        input.update_mouse_buttons(&mb, &s, &m);
                    }
                    glutin::WindowEvent::CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => {
                        input.update_mouse_pos(vec2(x as _, y as _));
                    }
                    _ => (),
                },
                glutin::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::DeviceEvent::Motion { axis, value } => {
                            // axis == 0 is X, 1 is Y
                            // println!("Motion: axis: {} value: {}", axis, value);
                            input.update_glutin_mouse_delta(axis, value as _);
                        }
                        glutin::DeviceEvent::Key(ki) => {
                            println!("Key: {:#?}", ki.scancode);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        if input.escape {
            break 'running;
        }

        if input.escape {
            break 'running;
        }

        clear(0.3, 0.0, 0.5, 1.0);

        perform_frame(time.delta_time(), &input, &mut camera);

        gl_context.swap_buffers().unwrap();

        time.wait_until_frame_target();
    }
}
