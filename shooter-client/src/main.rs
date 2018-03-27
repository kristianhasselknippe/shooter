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

#[macro_use] mod scripting;
mod project_format;
mod sprite;
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
mod fps_counter;
mod gui;

use gui::*;
use glutin::GlContext;
use sprite::*;
use project_format::*;
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
use scripting::lua::NativeLibraryProvider;

use libc::c_void;

use scripting::lua::LuaType;

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
    println!("Loading userdata libraries");
    game_state.register_native_library(&Camera::get_native_library());
    game_state.register_native_library(&GameState::get_native_library());
    game_state.register_native_library(&Entity::get_native_library());
    game_state.register_native_library(&Input::get_native_library());
    println!("Done loading userdata libraries");
    game_state.new_module(&Path::new("scripts/debug.lua"));
    game_state.new_module(&Path::new("scripts/math/vec3.lua"));
    game_state.new_module(&Path::new("scripts/math/vec2.lua"));
    game_state.new_module(&Path::new("scripts/math/constants.lua"));
    game_state.new_module(&Path::new("scripts/math/quat.lua"));
    game_state.new_module(&Path::new("scripts/math/utils.lua"));
    game_state.new_module(&Path::new("scripts/math/mat4.lua"));
    game_state.new_module(&Path::new("scripts/math/color.lua"));
    game_state.new_module(&Path::new("scripts/math/intersect.lua"));
    game_state.new_module(&Path::new("scripts/math/mesh.lua"));
    game_state.new_module(&Path::new("scripts/math/octree.lua"));
    game_state.new_module(&Path::new("scripts/math/simplex.lua"));
    game_state.new_module(&Path::new("scripts/helpers.lua"));
    game_state.new_module(&Path::new("scripts/globals.lua"));
    game_state.new_module(&Path::new("scripts/scene.lua"));
    game_state.new_module(&Path::new("scripts/main.lua"));
   
    let mut input = Input::new();
    game_state.register_global_pointer("InputRef", unsafe { c_ref_to_void!(&input) });
    
    let scene = load_from_file(Path::new("scenes/scene1"));

    let player_ref = game_state.new_entity("player");
    let player_script = game_state.register_script(Path::new("scripts/player.lua"), &player_ref);

    //performance testing
    /*for i in 0..100 {
        let player_ref = game_state.new_entity_with_pos("player", Vector2::new(i as f32 * 2.0, i as f32 * 2.0));
        let player_script = game_state.script_engine.new_behavior_script_from_path("player", Path::new("scripts/player.lua"));
        game_state.ecs.add_script(&player_ref, &player_script);
    }*/
        
    game_state.call_script_function("debug_scripts", &[]);
    
    let camera_ref = game_state.new_entity("camera");
    let camera_script = game_state.register_script(Path::new("scripts/camera.lua"), &camera_ref);
    camera_script.set_field("player", &LuaType::LightUserdata(unsafe {c_ref_to_void!(&camera_ref) }));
    
    let text = Text::new("this is some text", &draw_context);


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
        
        game_state.pre_update();

        if input.escape {
            break 'running;
        }

        game_state.update_entities(dt as f64);

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
