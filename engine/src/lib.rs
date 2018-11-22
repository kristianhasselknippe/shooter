extern crate alga;
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
extern crate nalgebra as na;
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
pub mod fps_counter;
pub mod gui;
pub mod input;
pub mod time;

use alga::general::Inverse;
use camera::*;
use drawing::*;
use fps_counter::*;
use glutin::{
    dpi::LogicalPosition, dpi::LogicalSize, dpi::PhysicalSize, ContextBuilder, EventsLoop,
    GlContext, GlWindow, WindowBuilder,
};
use gui::imgui::*;
use input::*;
use mesh::model::Model;
use na::{zero, Isometry3, Vector3};
use shader::*;
use time::*;
use utils::gl::*;

pub fn start_event_loop(
    mut events_loop: EventsLoop,
    mut window_size: (i32, i32),
    dpi_factor: f32,
    gl_window: GlWindow,
    body: &Fn(f32, Input, Gui),
) {
    let mut time = Time::new(60);
    let mut running = false;
    let mut dt = 0.0;
    let mut input = Input::new();
    let mut gui = Gui::new(
        window_size.0 as f32 * dpi_factor,
        window_size.1 as f32 * dpi_factor,
    );
    let mut camera = Camera::new_perspective(
        16.0 / 9.0,
        3.14 / 4.0,
        1.0,
        1000.0,
        na::Point3::new(0.0, 0.0, 8.0),
    );

    // let program = ShaderProgram::create_program("default");
    let mut program = ShaderProgram::create_program("default");

    //let model = Model::load_from_wavefront_file("quad.obj").unwrap();
    //let al = Model::load_from_wavefront_file("al.obj").unwrap();
    //let sphere = Model::load_from_wavefront_file("sphere.obj").unwrap();

    let bow = GameObject::new(
        "Bow",
        Model::load_from_wavefront_file("Bow/Bow.obj").unwrap(),
        Vector3::new(0.0, 0.0, 0.0),
    );
    let bow2 = GameObject::new(
        "Bow2",
        Model::load_from_wavefront_file("Bow2/Bow.obj").unwrap(),
        Vector3::new(40.0, 0.0, 0.0),
    );

    let mut game_objects = vec![bow, bow2];

    println!("Window size: {},{}", window_size.0, window_size.1);

    let mut fps_counter = FpsCounter::new();

    println!("DPI: {}", dpi_factor);
    viewport(
        (window_size.0 as f32 * dpi_factor) as i32,
        (window_size.1 as f32 * dpi_factor) as i32,
    );

    let mut fps = "FPS: 0".to_string();

    let mut world = specs::World::new();
    //world.register::<Position>();

    'running: while running {
        let dt = time.delta_time() as f32;

        input.reset_mouse_delta();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        gui.add_input_character(c);
                    }
                    glutin::WindowEvent::Resized(LogicalSize {
                        width: w,
                        height: h,
                    }) => {
                        println!("New Window size: {},{} - dpi: {}", w, h, dpi_factor);
                        window_size = (w as i32, h as i32);

                        let width = window_size.0 as f32 * dpi_factor;
                        let height = window_size.1 as f32 * dpi_factor;

                        gl_window.resize(PhysicalSize::new(width as f64, height as f64));
                        viewport(width as i32, height as i32);
                        gui.set_display_size((width, height));
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
                        input.update_mouse_pos(na::Vector2::new(x as _, y as _));
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
    }
    body(dt, input, gui);
    time.wait_until_frame_target();
}
