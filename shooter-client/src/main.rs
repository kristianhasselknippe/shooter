extern crate shooter_common;
extern crate sdl2;
extern crate gl;
extern crate nalgebra as na;
extern crate image;
extern crate rusttype;

mod shader;
mod mesh;
mod drawing;
mod transform;
mod entity;
mod texture;
mod text;
mod input;
mod scene;

use shader::*;
use mesh::*;
use drawing::*;
use texture::*;
use entity::*;
use text::*;
use input::*;
use scene::*;

use std::path::Path;

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

    let mut texture_atlas = TextureAtlas::new();


    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img1.png")));
    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img2.png")));
//    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img3.png")));
    //    texture_atlas.add_texture(MemoryTexture::from_png(Path::new("assets/img4.png")));

    texture_atlas.pack_and_draw(&mut draw_context);

    let program = ShaderProgram::create_program("default");

    texture_atlas.bind(&draw_context);

    /*let mut m = Mesh::create_quad();
    m.draw_now();*/

    let mut batch = Batch::new();
    batch.write_mesh(&Mesh::create_quad());
    batch.write_mesh(&Mesh::create_rect(0.3,0.3));
    batch.update_data();
    batch.draw();

    canvas.present();
    println!("We've presented");
    'running: loop {
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } |
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        /*draw_context.bind();

        draw_context.clear((1.0,0.0,1.0,1.0));

        draw_context.draw();
        draw_context.unbind();*/




        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
