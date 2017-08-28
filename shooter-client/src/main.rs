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
mod texture;
mod text;
mod camera;
mod entities;
mod time;
mod input;

use shader::*;
use mesh::*;
use drawing::*;
use texture::*;
use entities::*;
use text::*;
use input::*;
use camera::*;
use time::*;
use input::*;

use std::path::Path;
use na::*;
use alga::linear::Transformation;
use gl::types::*;

use std::cell::RefCell;
use std::rc::Rc;

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

    texture_atlas.bind(&draw_context);

    /*let mut m = Mesh::create_quad();
    m.draw_now();*/

    let mut batch = Batch::new();

    let mut player_sprite = Sprite::from_png(Path::new("assets/mario.png"), 5.0, 5.0);
    let mut background_sprite = Sprite::from_png(Path::new("assets/overworld.png"), 15.0, 15.0);

    let mut time = Time::new(60);

    let mut events = sdl_context.event_pump().unwrap();

    let input = Rc::new(RefCell::new(Input::new(events)));

    let mut game_state = GameState::new();

    let player_entity = game_state.add_entity(&Rc::new(RefCell::new((Entity::new(Vector2::new(0.0,0.0))))));
    let player_controller = Rc::new(RefCell::new(PlayerController::new(&input, 30.0, 20.0)));
    game_state.add_component(&player_controller, &player_entity);

    let mut camera = Camera::new_orthographic(50.0,50.0);
    let mut camera_controller = Rc::new(RefCell::new(PlayerCamera::new(&player_entity, camera, &input)));
    let mut camera_entity = game_state.add_entity(&Rc::new(RefCell::new(Entity::new(Vector2::new(0.0,0.0)))));
    game_state.add_component(&camera_controller, &camera_entity);



    let text = Text::new("this is some text", &draw_context);

    'running: loop {
        let dt = time.delta_time() as f32;
        {
            input.borrow_mut().update_sdl_input();
        }

        if input.borrow().escape {
            break 'running;
        }

        game_state.update(dt);

        //update player sprite position since they are not yet connected
        let p_entity = game_state.get_entity(&player_entity);
        player_sprite.pos.x = p_entity.pos.x;
        player_sprite.pos.y = p_entity.pos.y;

        draw_context.clear((1.0,0.0,1.0,1.0));

        let cam_entity = game_state.get_entity(&camera_entity);

        let view = Matrix4::new_translation(&Vector3::new(-cam_entity.pos.x,-cam_entity.pos.y,cam_entity.pos.z));
        let projection = camera_controller.borrow().camera_matrix(&game_state);
        let camera_matrix = projection * view;

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

        text.draw();


        canvas.present();

        time.wait_until_frame_target();
    }
}
