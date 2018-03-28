#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate glutin;
extern crate gl;
extern crate nalgebra as na;
extern crate alga;
extern crate image;
extern crate rusttype;
extern crate time as t;
extern crate libc;
extern crate ordered_float as of;

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
use drawing::DrawContext;

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
        gl::Disable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
    };

    let camera = Camera::new_orthographic(60.0, 60.0);
    let mut dc = DrawContext::new(window_size.0, window_size.1, camera);

    // let program = ShaderProgram::create_program("default");
    let program = ShaderProgram::create_program("default");

    let mut models = Model::load_from_wavefront_file("al.obj").unwrap();
    println!("Model: {:?}", models);

    let mut time = Time::new(60);

    let mut input = Input::new();

    println!("Window size: {},{}", window_size.0, window_size.1);

    let mut fps_counter = FpsCounter::new();
    let mut running = true;

    let camera = Camera::new_perspective(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);
    let mut camera_pos = na::Point3::<f32>::new(0.0, 0.0, 1.0);

    dc.bind();

    clear(0.3, 0.0, 0.5, 1.0);

    program.use_program();
    // program.set_mat4("mvp", &model_view_projection);

    println!("Models length: {}", models.len());
    for mut m in &mut models {
        m.draw();
    }

    println!("Swapping \n\n");

    viewport(window_size.0 as i32, window_size.1 as i32);

    gl_window.swap_buffers().unwrap();

    'running: while running {
        let dt = time.delta_time() as f32;

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
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
                    }
                }
                _ => (),
            }
        });

/*        let input_vector = input.normalized_input_vector();

        // camera_pos += na::Vector3::new(input_vector.x, 0.0, -input_vector.y) * dt;

        // Our object is translated along the x axis.
        let model = na::Isometry3::new(na::Vector3::new(0.0, 0.0, 0.0), na::zero());

        let eye = camera_pos;
        let target = na::Point3::new(0.0, 0.0, -1.0);
        let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());

        let model_view_projection = camera.projection * (view * model).to_homogeneous();

        if input.escape {
            break 'running;
        }

        clear(0.3, 0.0, 0.5, 1.0);

        program.use_program();
        program.set_mat4("mvp", &model_view_projection);

        println!("Models length: {}", models.len());
        for mut m in &mut models {
            m.draw();
        }

        println!("Swapping \n\n");*/

        
        //gl_window.swap_buffers().unwrap();

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
