#![allow(dead_code)]

extern crate alga;
extern crate engine;
extern crate gl;
extern crate glutin;
extern crate nalgebra as na;
extern crate specs;
extern crate specs_derive;

/*use nc::{
    shape::{ShapeHandle},
    world::{CollisionWorld,CollisionGroups,GeometricQueryType}
};*/
use engine::start_event_loop;
use glutin::{ContextBuilder, EventsLoop, GlContext, GlWindow, WindowBuilder};
use na::{zero, Isometry3, Vector3};

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

    let dpi_factor = gl_window.hidpi_factor();

    start_event_loop(
        events_loop,
        window_size,
        dpi_factor | dt,
        gl_window,
        input | {
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
            //gui.new_frame();

            //gui.begin("Info", true);

            if let Some(_fps) = fps_counter.update(dt as _) {
                fps = _fps;
            }
            //gui.text(&fps);

            clear(0.3, 0.0, 0.5, 1.0);

            gui.draw_object_list(game_objects.iter().map(|x| x.name.as_str()));

            for mut o in &mut game_objects {
                //GUI

                //gui.text(&format!("Model {}", o.name));
                //gui.drag_float3(&format!("Position##{}", o.name), &mut o.position, 0.2, -10000.0, 10000.0);

                let model_isom = Isometry3::new(o.position, zero()).to_homogeneous();
                let model_view = camera.view() * model_isom;

                let m_inv = model_isom
                    .fixed_slice::<na::U3, na::U3>(0, 0)
                    .clone_owned()
                    .inverse();

                let mv_inv = model_view
                    .fixed_slice::<na::U3, na::U3>(0, 0)
                    .clone_owned()
                    .inverse();

                let mut dc = o.get_draw_call(&mut program);
                let mut bound_dc = dc.bind();
                bound_dc.set_mat3("m_inv", &m_inv);
                bound_dc.set_mat3("mv_inv", &mv_inv);
                bound_dc.set_mat4("model", &model_isom);
                bound_dc.set_mat4("view", &camera.view());
                bound_dc.set_mat4("projection", &camera.projection);
                bound_dc.set_int("diffuseMap", 0);

                bound_dc.perform();
            }

            gui.render(
                window_size.0 as f32 * dpi_factor,
                window_size.1 as f32 * dpi_factor,
            );

            gl_window.swap_buffers().unwrap();
        },
    )
}
