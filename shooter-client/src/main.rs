#![allow(dead_code)]

extern crate shooter_common;
extern crate sdl2;
extern crate gl;
extern crate nalgebra as na;
extern crate alga;
extern crate image;
extern crate rusttype;
extern crate time as t;
extern crate libc;
extern crate ordered_float as of;

mod scene;
mod shader;
mod mesh;
mod drawing;
mod transform;
mod texture;
mod text;
mod camera;
mod entities;
mod game_state;
mod time;
mod input;
mod scripting;
mod fps_counter;

use shader::*;
use drawing::*;
use texture::*;
use entities::*;
use text::*;
use camera::*;
use time::*;
use input::*;
use fps_counter::*;
use game_state::*;

use scripting::lua::LuaType;

use std::path::Path;
use na::*;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {

    let window_size = (800,600);


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let swap_interval = video_subsystem.gl_get_swap_interval();
    println!("Swap interval: {}", swap_interval);
    

    let gl_attr = video_subsystem.gl_attr();
    
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_major_version(3);
    gl_attr.set_context_minor_version(3);

    let window = video_subsystem.window("Shooter", window_size.0, window_size.1)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let major = window.subsystem().gl_attr().context_major_version();
    let minor = window.subsystem().gl_attr().context_minor_version();
    println!("Major {}, Minor {}", major, minor);
    
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    


    let mut draw_context = DrawContext::new(window_size.0, window_size.1);

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
    program.use_program();

    texture_atlas.bind();

    /*let mut m = Mesh::create_quad();
    m.draw_now();*/

    //let batch = Batch::new();

    let mut player_sprite = Sprite::from_png(Path::new("assets/mario.png"), 5.0, 5.0);
    let background_sprite = Sprite::from_png(Path::new("assets/overworld.png"), 15.0, 15.0);

    let mut time = Time::new(60);

    let events = sdl_context.event_pump().unwrap();

    let mut input = Input::new(events);

    let mut game_state = GameState::new();

    game_state.new_entity("player");
    game_state.new_entity("camera");

    let text = Text::new("this is some text", &draw_context);


    //unsafe { gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32) };

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    };
    
    let mut fps_counter = FpsCounter::new();
    'running: loop {
        let dt = time.delta_time();

        let entities = game_state.get_entities();

        let dt = dt as f32;
        game_state.pre_update();
        {
            input.update_sdl_input();
            game_state.update_input(&input);
        }

        if input.escape {
            break 'running;
        }

        game_state.update_entities(dt as f64);

        //update player sprite position since they are not yet connected
        let p_entity = game_state.get_entity("player").unwrap();
        player_sprite.pos.x = p_entity.pos.x;
        player_sprite.pos.y = p_entity.pos.y;
        player_sprite.rot = p_entity.rot;

        draw_context.clear((1.0,0.0,1.0,1.0));

        let cam_entity = game_state.get_entity("camera").unwrap();
        let view = Matrix4::new_translation(&Vector3::new(-cam_entity.pos.x,-cam_entity.pos.y,cam_entity.pos.z));

        let mut camera: *mut Camera = *game_state.script_engine.lua.get_userdata::<Camera>("Camera");
        let projection = unsafe { (*camera).camera_matrix() };
        let camera_matrix = projection * view;


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

        background_sprite.draw(&camera_matrix);
        player_sprite.draw(&camera_matrix);




        //window.gl_swap_window();
        canvas.present();
        

        time.wait_until_frame_target();
        fps_counter.update(dt);
    }
}
