#![allow(dead_code)]

extern crate alga;
extern crate gl;
extern crate glutin;
extern crate image as img;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate nalgebra as na;
extern crate ncollide3d as nc;
extern crate ordered_float as of;
extern crate rusttype;
extern crate time as t;
extern crate itertools;
extern crate imgui_sys as imgui;

mod utils;
mod scene;
mod shader;
mod collision;
mod mesh;
mod transform;
mod drawing;
mod image;
// mod text;
mod camera;
mod entities;
mod time;
mod input;
mod fps_counter;
mod gui;

use glutin::{GlContext,GlWindow,WindowBuilder,EventsLoop,ContextBuilder};
use shader::*;
// use text::*;
use mesh::model::Model;
use camera::*;
use time::*;
use input::*;
use fps_counter::*;
use utils::gl::*;
use drawing::*;
use alga::general::Inverse;

use nc::{
    shape::{ShapeHandle},
    world::{CollisionWorld,CollisionGroups,GeometricQueryType}
};
use na::{Isometry3,Vector3,zero};

use gui::*;

fn main() {
    let mut window_size = (800, 600);

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(window_size.0, window_size.1);
    let context = ContextBuilder::new().with_vsync(true);
    let gl_window = GlWindow::new(window, context, &events_loop).unwrap();
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

    // let program = ShaderProgram::create_program("default");
    let program = std::rc::Rc::new(ShaderProgram::create_program("default"));

    //let model = Model::load_from_wavefront_file("quad.obj").unwrap();
    //let al = Model::load_from_wavefront_file("al.obj").unwrap();
    //let sphere = Model::load_from_wavefront_file("sphere.obj").unwrap();

    let bow = GameObject::new(
        "Bow",
        Model::load_from_wavefront_file("Bow/Bow.obj").unwrap(),
        Vector3::new(0.0,0.0,0.0)
    );
    let bow2 = GameObject::new(
        "Bow2",
        Model::load_from_wavefront_file("Bow2/Bow.obj").unwrap(),
        Vector3::new(40.0,0.0,0.0)
    );

    let mut game_objects = vec![bow, bow2];

    let mut time = Time::new(60);

    println!("Window size: {},{}", window_size.0, window_size.1);

    let mut fps_counter = FpsCounter::new();

    let dpi_factor = gl_window.hidpi_factor();
    println!("DPI: {}", dpi_factor);
    viewport((window_size.0 as f32 * dpi_factor) as i32,
             (window_size.1 as f32 * dpi_factor) as i32);

    let mut input = Input::new();

    let mut gui = Gui::init_gui(
        window_size.0 as f32 * dpi_factor,
        window_size.1 as f32 * dpi_factor,
    );

    let mut running = true;

    'running: while running {
        let dt = time.delta_time() as f32;

        input.reset_mouse_delta();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        println!("Received char: {}", c);
                        gui.add_input_character(c);
                    },
                    glutin::WindowEvent::Resized(w, h) => {
                        println!("New Window size: {},{} - dpi: {}", w, h, dpi_factor);
                        window_size = (w,h);

                        let width = window_size.0 as f32 * dpi_factor;
                        let height = window_size.1 as f32 * dpi_factor;

                        gl_window.resize(width as u32, height as u32);
                        viewport(width as i32, height as i32);
                        gui.set_display_size((width, height));
                        camera.set_aspect(width/height);
                    },
                    glutin::WindowEvent::KeyboardInput { input: i, .. } => {
                        input.update_glutin_keyboard_input(&i);
                    },
                    glutin::WindowEvent::MouseInput {
                        state: s,
                        button: mb,
                        modifiers: m,
                        ..
                    } => {
                        input.update_mouse_buttons(&mb, &s, &m);
                    },
                    glutin::WindowEvent::CursorMoved {
                        position: (x, y),
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

        let mut speed = 10.0;
        if input.shift {
            speed *= 2.0;
        }
        if input.mouse_right {
            //gl_window.set_cursor_state(glutin::CursorState::Grab);

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
            //gl_window.set_cursor_state(glutin::CursorState::Normal);
        }

        if input.escape {
            running = false;
        }
        gui.update_input(&input, dt);
        gui.new_frame();

        gui.begin("Models", true);

        clear(0.3, 0.0, 0.5, 1.0);

        let mut vao = VertexArray::new();

        let mut vertex_spec = VertexSpec::new(vec![
            VertexAttribute::new(0, gl::FLOAT, 3, false),
            VertexAttribute::new(1, gl::FLOAT, 3, false),
            VertexAttribute::new(2, gl::FLOAT, 3, false),
        ]);

        vao.bind();

        for mut o in &mut game_objects {
            //GUI
            let mut m = &mut o.model;
            gui.text(&format!("Model {}", o.name));
            gui.drag_float3(&format!("Position##{}", o.name), &mut o.position, 0.2, -10000.0, 10000.0);

            m.bind();

            //A vertex buffer has to be bound before enabling the vertex_spec!
            vertex_spec.enable();

            program.use_program();

            let model = o.position.to_homogeneous();
            let model_isom = Isometry3::new(o.position, zero()).to_homogeneous();
            let model_view = camera.view() * model_isom;

            let m_inv = model_isom
                .fixed_slice::<na::U3,na::U3>(0,0)
                .clone_owned()
                .inverse();

            let mv_inv = model_view
                .fixed_slice::<na::U3,na::U3>(0,0)
                .clone_owned()
                .inverse();

            program.set_mat3("m_inv", &m_inv);
            program.set_mat3("mv_inv", &mv_inv);
            program.set_mat4("model", &model_isom);
            program.set_mat4("view", &camera.view());
            program.set_mat4("projection", &camera.projection);
            program.set_int("diffuseMap", 0);

            unsafe {
                enable(Capability::CullFace);
                enable(Capability::DepthTest);
            }

            check_gl_errors();
            draw_triangles(m.num_indices, m.index_type);

        }
        vao.unbind();
        gui.end();


        gui.render(window_size.0 as f32 * dpi_factor, window_size.1 as f32 * dpi_factor);

        gl_window.swap_buffers().unwrap();

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
