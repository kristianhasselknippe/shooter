#![allow(dead_code)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
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
//mod text;
mod camera;
mod entities;
mod time;
mod input;
mod fps_counter;

use glutin::GlContext;
use shader::*;
//use text::*;
use mesh::model::Model;
use camera::*;
use time::*;
use input::*;
use fps_counter::*;
use na::*;

fn main() {

    let window_size = (800,600);
    
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(window_size.0, window_size.1);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }
    
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    };

    let camera = Camera::new_orthographic(60.0, 60.0);

    //let program = ShaderProgram::create_program("default");
    let program = ShaderProgram::from_fragments("TexCoord = tex_coord;\n
                                                 gl_Position = vec4(position.x, position.y, position.z, 1.0);",
"float distance = 1.0 - distance(vec2(0.0,0.0), TexCoord * 2.0 - 1.0);
color = vec4(distance,distance,distance,1.0);");


    /*let model = Model::load_from_wavefron_file("al.obj");
    assert!(mesh.is_ok());*/

    let mut time = Time::new(60);
   
    let mut input = Input::new();

    unsafe { gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32) };
      
    let mut fps_counter = FpsCounter::new();
    let mut running = true;
    
    'running: while running {
        let dt = time.delta_time();
        
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => { running = false; },
                    glutin::WindowEvent::Resized(w, h) => {
                        gl_window.resize(w, h);
                        unsafe { gl::Viewport(0, 0, w as i32, h as i32) };  
                    },
                    glutin::WindowEvent::KeyboardInput { input: i, .. } => {
                            input.update_glutin_input(&i);
                        },
                    _ => (),
                },
                _ => ()
            }
        });
        
        if input.escape {
            break 'running;
        }

        //draw_context.clear((0.5,0.0,1.0,1.0));

        /*let cam_entity = game_state.get_entity(&camera_ref).unwrap();
        let view = Matrix4::new_translation(&Vector3::new(-cam_entity.pos.x,-cam_entity.pos.y,cam_entity.pos.z));

        let projection = camera.camera_matrix();
        let camera_matrix = projection * view;*/
        
        //draw_context.set_camera_matrix(camera_matrix);

        

        gl_window.swap_buffers().unwrap();
        
        

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
