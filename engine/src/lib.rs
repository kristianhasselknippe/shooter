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
extern crate ordered_float as of;
extern crate rusttype;
extern crate time as t;

extern crate specs;
extern crate specs_derive;

pub mod drawing;
pub mod image;
pub mod mesh;
pub mod scene;
pub mod shader;
pub mod transform;
pub mod utils;
// mod text;
pub mod camera;
pub mod default_init;
pub mod fps_counter;
//pub mod gui;
pub mod input;
pub mod time;

use camera::*;
use default_init::{init_defaults, EngineContext};
use drawing::*;
use fps_counter::*;
use glm::*;
use glutin::{
    dpi::LogicalPosition, dpi::LogicalSize, dpi::PhysicalSize, Context, ContextBuilder,
    ContextCurrentState, EventsLoop, PossiblyCurrent, Window, WindowBuilder, WindowedContext,
};
//use gui::imgui::*;
use input::*;
use mesh::model::Model;
use shader::*;
use specs::prelude::*;
use time::*;
use utils::gl::*;

pub fn init_gl_window(window_size: (i32, i32)) -> (EventsLoop, WindowedContext<PossiblyCurrent>) {
    let el = EventsLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    unsafe {
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
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

    (el, windowed_context)
}

pub fn start_event_loop() {
    let mut window_size = (800, 600);
    let (mut events_loop, gl_context) = init_gl_window(window_size);

    let window = gl_context.window();

    println!("Window size: {},{}", window_size.0, window_size.1);
    let dpi_factor = window.get_hidpi_factor();
    println!("DPI: {}", dpi_factor);
    viewport(
        (window_size.0 as f32 * dpi_factor as f32) as i32,
        (window_size.1 as f32 * dpi_factor as f32) as i32,
    );

    let EngineContext {
        mut camera,
        mut program,
        mut game_objects,
        mut time,
        mut fps_counter,
        mut input,
    } = init_defaults(window_size, dpi_factor as f32);

    let mut running = true;

    let mut fps = "FPS: 0".to_string();

    let mut world = World::new();
    //world.register::<Position>();

    'running: while running {
        let dt = time.delta_time() as f32;

        input.reset_mouse_delta();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        //gui.add_input_character(c);
                    }
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
                        camera.set_aspect(width / height);
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

        let mut speed = 10.0;
        if input.shift {
            speed *= 2.0;
        }
        if input.mouse_right {
            //gl_context.set_cursor_icon(glutin::CursorIcon::Grab);

            camera.yaw += input.mouse_delta.x / 125.0;
            camera.pitch -= input.mouse_delta.y / 150.0;

            if input.forward {
                camera.move_forward(dt * -speed);
            }
            if input.backward {
                camera.move_forward(dt * speed);
            }
            if input.left {
                camera.move_right(dt * -speed);
            }
            if input.right {
                camera.move_right(dt * speed);
            }
            if input.up {
                camera.move_up(dt * speed);
            }
            if input.down {
                camera.move_up(dt * -speed);
            }
        } else {
            //gl_context.set_cursor_icon(glutin::CursorIcon::Normal);
        }

        if input.escape {
            running = false;
        }

        //gui.new_frame();

        //gui.begin("Info", true);

        if let Some(_fps) = fps_counter.update(dt as _) {
            fps = _fps;
        }
        //gui.text(&fps);

        clear(0.3, 0.0, 0.5, 1.0);

        for mut o in &mut game_objects {
            //GUI

            //gui.text(&format!("Model {}", o.name));
            //gui.drag_float3(&format!("Position##{}", o.name), &mut o.position, 0.2, -10000.0, 10000.0);

            let model_isom = translation(&o.position);
            let model_view = camera.view() * model_isom;

            let m_inv = inverse(
                &model_isom
                    .fixed_slice::<glm::U3, glm::U3>(0, 0)
                    .clone_owned(),
            );

            let mv_inv = inverse(
                &model_view
                    .fixed_slice::<glm::U3, glm::U3>(0, 0)
                    .clone_owned(),
            );

            let mut dc = o.get_draw_call(&mut program);
            let mut bound_dc = dc.bind();
            bound_dc.set_mat3("m_inv", &m_inv);
            bound_dc.set_mat3("mv_inv", &mv_inv);
            bound_dc.set_mat4("model", &model_isom);
            bound_dc.set_mat4("view", &camera.view());
            bound_dc.set_mat4("projection", &camera.projection);
            bound_dc.set_int("diffuseMap", 0);

            bound_dc.perform();
        }

        gl_context.swap_buffers().unwrap();

        time.wait_until_frame_target();
    }
}
