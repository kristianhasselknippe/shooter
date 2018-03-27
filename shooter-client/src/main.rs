#![allow(dead_code)]

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
mod sprite;
mod scene;
mod shader;
mod mesh;
mod drawing;
mod transform;
mod texture;
//mod text;
mod camera;
mod entities;
mod game_state;
mod time;
mod input;
mod fps_counter;
mod gui;

use gui::*;
use glutin::GlContext;
use sprite::*;
use shader::*;
use drawing::*;
use texture::*;
//use text::*;
use camera::*;
use time::*;
use input::*;
use fps_counter::*;
use game_state::*;

use std::path::Path;
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

    let mut draw_context = DrawContext::new(window_size.0, window_size.1, camera.camera_matrix());
    draw_context.bind();

    let mut texture_atlas = TextureAtlas::new();

    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img1.png")));
    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img2.png")));

    texture_atlas.pack_and_draw(&mut draw_context);

    //let program = ShaderProgram::create_program("default");
    let program = ShaderProgram::from_fragments("TexCoord = tex_coord;\n
                                                 gl_Position = vec4(position.x, position.y, position.z, 1.0);",
"float distance = 1.0 - distance(vec2(0.0,0.0), TexCoord * 2.0 - 1.0);
color = vec4(distance,distance,distance,1.0);");

    draw_context.add_shader_program("some_program", program);
    draw_context.add_shader_program("sprite", ShaderProgram::create_program("sprite"));
    draw_context.add_shader_program("solid_color", ShaderProgram::create_program_from_vert_frag("screen_space", "solid_color"));
    draw_context.use_shader_program("some_program");



    texture_atlas.bind();

    /*let mut m = Mesh::create_quad();
    m.draw_now();*/

    //let batch = Batch::new();

    let mut player_sprite = Sprite::from_png(Path::new("assets/mario.png"), 5.0, 5.0);
    let background_sprite = Sprite::from_png(Path::new("assets/overworld.png"), 15.0, 15.0);

    let mut time = Time::new(60);

    let mut game_state = GameState::new("this is the one i made");
   
    let mut input = Input::new();

    let player_ref = game_state.new_entity("player");
    
    let camera_ref = game_state.new_entity("camera");

    //let text = Text::new("this is some text", &draw_context);

    let mut gui = Panel::new(Vector2::new(100.0,100.0), Vector2::new(100.0,50.0));
    gui.add_drawable(Box::new(Shape::new_with_color(Vector4::new(1.0,0.0,0.0,1.0))));
    gui.add_drawable(Box::new(Shape::new_with_color(Vector4::new(1.0,1.0,0.0,1.0))));
        
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
        
        //update player sprite position since they are not yet connected
        let p_entity = game_state.get_entity(&player_ref).unwrap();
        player_sprite.pos.x = p_entity.pos.x;
        player_sprite.pos.y = p_entity.pos.y;
        player_sprite.rot = p_entity.rot;

        draw_context.clear((0.5,0.0,1.0,1.0));

        let cam_entity = game_state.get_entity(&camera_ref).unwrap();
        let view = Matrix4::new_translation(&Vector3::new(-cam_entity.pos.x,-cam_entity.pos.y,cam_entity.pos.z));

        let projection = camera.camera_matrix();
        let camera_matrix = projection * view;
        draw_context.set_camera_matrix(camera_matrix);

        //Drawing
        /*for y in 0..10 {
            for x in 0..10 {
                let w = 1.0;
                let h = 1.0;

                let pos = Point3::new(x as f32,y as f32,0.0);
                let size = Vector3::new(w,h,1.0);

                let pos = camera_matrix.transform_point(&pos);
                let size = camera_matrix.transform_vector(&size);

                batch.write_mesh(&Mesh::create_from_topleft_bottomright((pos.x,pos.y), (pos.x + size.x, pos.y + size.y)));
            }
        }*/

        //batch.update_data();
        //batch.draw();

        background_sprite.draw(&draw_context);
        player_sprite.draw(&draw_context);

        gui.draw(&draw_context);

        gl_window.swap_buffers().unwrap();
        
        

        time.wait_until_frame_target();
        fps_counter.update(dt as f32);
    }
}
