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
use alga::general::Inverse;

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
        3.14 / 4.0,
        1.0,
        1000.0,
        na::Point3::new(0.0, 0.0, 8.0),
    );
    let mut dc = DrawContext::new(window_size.0, window_size.1);

    // let program = ShaderProgram::create_program("default");
    let program = std::rc::Rc::new(ShaderProgram::create_program("default"));

    //let model = Model::load_from_wavefront_file("quad.obj").unwrap();
    let al = Model::load_from_wavefront_file("al.obj").unwrap();
    //let sphere = Model::load_from_wavefront_file("sphere.obj").unwrap();
    let bow = Model::load_from_wavefront_file("Bow/Bow.obj").unwrap();

    let mut draw_calls = Vec::new();
    draw_calls.push(DrawCall::new(
        program.clone(),
        bow,
        vec![
            VertexAttribute::new(0, gl::FLOAT, 3),
            VertexAttribute::new(1, gl::FLOAT, 3),
            VertexAttribute::new(2, gl::FLOAT, 3),
        ],
        drawing::Transform::from_pos(na::Vector3::new(0.0,0.0,-8.0))
    ));
    draw_calls.push(DrawCall::new(
        program.clone(),
        al,
        vec![
            VertexAttribute::new(0, gl::FLOAT, 3),
            VertexAttribute::new(1, gl::FLOAT, 3),
            VertexAttribute::new(2, gl::FLOAT, 3),
        ],
        drawing::Transform::from_pos(na::Vector3::new(15.0,0.0,0.0))
    ));

    /*draw_calls.push(DrawCall::new(
        program.clone(),
        sphere,
        vec![
            VertexAttribute::new(0, gl::FLOAT, 3),
            VertexAttribute::new(1, gl::FLOAT, 3),
        ],
        drawing::Transform::from_pos(na::Vector3::new(10.0,0.0,0.0))
    ));*/

    let mut time = Time::new(60);

    let mut input = Input::new();

    println!("Window size: {},{}", window_size.0, window_size.1);

    let mut fps_counter = FpsCounter::new();
    let mut running = true;

    viewport(window_size.0 as i32, window_size.1 as i32);

    let mut accum = 0.0;

    let mut forward = false;
    let mut backward = false;
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;


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
                        if let Some(keycode) = i.virtual_keycode {
                            match keycode {
                                glutin::VirtualKeyCode::A => {
                                    left = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                glutin::VirtualKeyCode::D => {
                                    right = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                glutin::VirtualKeyCode::W => {
                                    forward = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                glutin::VirtualKeyCode::S => {
                                    backward = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                glutin::VirtualKeyCode::Q => {
                                    down = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                glutin::VirtualKeyCode::E => {
                                    up = if i.state == glutin::ElementState::Pressed { true } else { false };
                                },
                                _ => {},
                            }
                        }
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
                        },
                        glutin::DeviceEvent::Key(ki) => {
                            println!("Key: {:#?}", ki.scancode);
                        },
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        camera.yaw += mouseDelta.x / 125.0;
        camera.pitch += mouseDelta.y / 150.0;

        if forward {
            camera.move_forward(dt * -10.0);
        }
        if backward {
            camera.move_forward(dt * 10.0);
        }
        if left {
            camera.move_right(dt * -10.0);
        }
        if right {
            camera.move_right(dt * 10.0);
        }
        if up {
            camera.move_up(dt * 10.0);
        }
        if down {
            camera.move_up(dt * -10.0);
        }

        if input.escape {
            break 'running;
        }

        clear(0.3, 0.0, 0.5, 1.0);
        for mut d in &mut draw_calls {
            d.bind();
            let model = d.transform.matrix();
            let model_view = camera.view() * model;
            let model_view_projection = camera.camera_matrix() * model;

            let m_inv = model
                .fixed_slice::<na::U3,na::U3>(0,0)
                .clone_owned()
                .inverse();

            let mv_inv = model_view
                .fixed_slice::<na::U3,na::U3>(0,0)
                .clone_owned()
                .inverse();

            d.set_mat3("m_inv", &m_inv);
            d.set_mat3("mv_inv", &mv_inv);
            d.set_mat4("model", &d.transform.matrix());
            d.set_mat4("view", &camera.view());
            d.set_mat4("projection", &camera.projection);
            d.draw();
            d.unbind();
        }

        gl_window.swap_buffers().unwrap();

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
