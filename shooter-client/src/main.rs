#![allow(dead_code)]

extern crate alga;
extern crate gl;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate nalgebra as na;
extern crate ordered_float as of;
extern crate rusttype;
extern crate time as t;

mod utils;
mod scene;
mod shader;
mod mesh;
mod transform;
mod drawing;
// mod text;
mod camera;
mod entities;
mod time;
mod input;
mod fps_counter;

use glutin::GlContext;
use shader::*;
// use text::*;
use mesh::model::Model;
use camera::*;
use time::*;
use input::*;
use fps_counter::*;
use utils::gl::*;
use drawing::{DrawCall, DrawContext};

fn main() {
    let window_size = (800, 600);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(window_size.0, window_size.1);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    println!("GL version: {}", get_gl_version());

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    };

    let mut camera = Camera::new_perspective(
        16.0 / 9.0,
        3.14 / 2.0,
        1.0,
        1000.0,
        na::Point3::new(0.0, 0.0, 1.0),
    );
    let mut dc = DrawContext::new(window_size.0, window_size.1);

    // let program = ShaderProgram::create_program("default");
    let program = std::rc::Rc::new(ShaderProgram::create_program("default"));

    let model = Model::load_from_wavefront_file("al.obj").unwrap();

    let mut draw_calls = Vec::new();
    draw_calls.push(DrawCall::new(
        program.clone(),
        model,
        vec![
            VertexAttribute::new(0, gl::FLOAT, 3),
            VertexAttribute::new(1, gl::FLOAT, 3),
        ],
    ));

    let mut time = Time::new(60);

    let mut input = Input::new();

    println!("Window size: {},{}", window_size.0, window_size.1);

    let mut fps_counter = FpsCounter::new();
    let mut running = true;

    viewport(window_size.0 as i32, window_size.1 as i32);

    let mut accum = 0.0;

    'running: while running {
        let dt = time.delta_time() as f32;
        accum += dt;

        let mut mouseDelta = na::Vector2::new(0.0, 0.0);

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => {
                        running = false;
                    }
                    glutin::WindowEvent::Resized(w, h) => {
                        gl_window.resize(w, h);
                        unsafe { gl::Viewport(0, 0, w as i32, h as i32) };
                    }
                    glutin::WindowEvent::KeyboardInput { input: i, .. } => {
                        input.update_glutin_input(&i);
                    }
                    _ => (),
                },
                glutin::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::DeviceEvent::Motion { axis, value } => {
                            // axis == 0 is X, 1 is Y
                            // println!("Motion: axis: {} value: {}", axis, value);
                            if axis == 0 {
                                mouseDelta += na::Vector2::new(value as f32, 0.0);
                            } else {
                                mouseDelta -= na::Vector2::new(0.0, value as f32);
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        // let input_vector = input.normalized_input_vector();

        camera.yaw += mouseDelta.x / 125.0;
        camera.pitch += mouseDelta.y / 150.0;

        if input.escape {
            break 'running;
        }

        let model = na::Isometry3::new(
            na::Vector3::new(0.0, 0.0, -3.0),
            na::Vector3::new(0.0, accum.cos(), 0.0),
        );

        let model_view_projection = camera.camera_matrix() * model.to_homogeneous();

        clear(0.3, 0.0, 0.5, 1.0);

        for mut d in &mut draw_calls {
            d.set_mat4("mvp", &model_view_projection);
            dc.draw(&mut d);
        }

        gl_window.swap_buffers().unwrap();

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
