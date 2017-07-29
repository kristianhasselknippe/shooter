extern crate shooter_common;
extern crate sdl2;
extern crate gl;
extern crate nalgebra as na;
extern crate alga;
extern crate image;
extern crate rusttype;
extern crate time as t;

mod shader;
mod mesh;
mod drawing;
mod transform;
mod entity;
mod texture;
mod text;
mod input;
mod camera;
mod scene;
mod time;

use shader::*;
use mesh::*;
use drawing::*;
use texture::*;
use entity::*;
use text::*;
use input::*;
use scene::*;
use camera::*;
use time::*;

use std::path::Path;
use na::*;
use alga::linear::Transformation;
use gl::types::*;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {

    let window_size = (600,800);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Shooter", window_size.0, window_size.1)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current();

    let mut events = sdl_context.event_pump().unwrap();

    let mut draw_context = DrawContext::new(window_size.0, window_size.1);

    draw_context.bind();

    let mut camera = Camera::new_orthographic(50.0,50.0, Vector3::new(0.0,0.0,0.0));

    let mut texture_atlas = TextureAtlas::new();


    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img1.png")));
    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img2.png")));
//    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img3.png")));
    //    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img4.png")));

    texture_atlas.pack_and_draw(&mut draw_context);

    //let program = ShaderProgram::create_program("default");
    let program = ShaderProgram::from_fragments("TexCoord = tex_coord;\n
                                                 gl_Position = vec4(position.x, position.y, position.z, 1.0);",
"float distance = 1.0 - distance(vec2(0.0,0.0), TexCoord * 2.0 - 1.0);
color = vec4(distance,distance,distance,1.0);");
    program.use_program();

    texture_atlas.bind(&draw_context);

    /*let mut m = Mesh::create_quad();
    m.draw_now();*/



    let mut batch = Batch::new();

    println!("We've presented");


    let mut w_down = false;
    let mut a_down = false;
    let mut s_down = false;
    let mut d_down = false;

    let mut time = Time::new(60);

    'running: loop {
        let dt = time.delta_time() as f32;

        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } |
                sdl2::event::Event::Quit { .. } => break 'running,

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } => { w_down = true; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } => { a_down = true; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => { s_down = true; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => { d_down = true; },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), .. } => { w_down = false; },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), .. } => { a_down = false; },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), .. } => { s_down = false; },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), .. } => { d_down = false; },
                _ => {}
            }
        }

        let speed = 0.5;

        if w_down { camera.translate(Vector3::new( 0.0, -speed * dt,0.0)); }
        if a_down { camera.translate(Vector3::new( speed*dt, 0.0,0.0)); }
        if s_down { camera.translate(Vector3::new( 0.0, speed*dt ,0.0)); }
        if d_down { camera.translate(Vector3::new(-speed * dt,0.0,0.0)); }

        //        draw_context.clear((1.0,0.0,1.0,1.0));
        let camera_matrix = camera.camera_matrix();

        for y in 0..10 {
            for x in 0..10 {
                let w = 1.0;
                let h = 1.0;

                let pos = Point3::new(x as f32,y as f32,0.0);
                let size = Vector3::new(w,h,1.0);

                let pos = camera_matrix.transform_point(&pos);
                let size = camera_matrix.transform_vector(&size);

                batch.write_mesh(&Mesh::create_from_topleft_bottomright((pos.x,pos.y), (pos.x + size.x, pos.y + size.y)));
            }
        }
        batch.update_data();
        batch.draw();

        canvas.present();

        time.wait_until_frame_target();
    }
}
