#![allow(dead_code)]

extern crate alga;
extern crate engine;
extern crate glutin;
extern crate nalgebra_glm as glm;
extern crate specs;
extern crate specs_derive;

use engine::camera::*;
use engine::drawing::*;
use engine::input::*;
use engine::shader::*;
use engine::start_event_loop;

fn main() {
    let mut game_objects: Vec<GameObject> = vec![];

    // TODO: This must be run after GL is initialized
    //let mut program = ShaderProgram::create_program("default");

    start_event_loop(&mut |dt, input: &Input, camera: &mut Camera<f64>| {
        let mut speed = 10.0;
        if input.shift {
            speed *= 2.0;
        }

        if input.mouse_right {
            //gl_context.set_cursor_icon(glutin::CursorIcon::Grab);

            camera.yaw += (input.mouse_delta.x / 125.0) as f64;
            camera.pitch -= (input.mouse_delta.y / 150.0) as f64;

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
        }

        /*for o in &mut game_objects {
            let model_isom = translation(&o.position);
            let model_view = camera.view() * model_isom;

            let m_inv = inverse(
                &model_isom
                    .fixed_slice::<glm::U3, glm::U3>(0, 0)
                    .clone_owned(),
            );

            let mv_inv = inverse(
                &model_view
                    .fixed_slice::<glm::U3, glm::U3>(0, 0)
                    .clone_owned(),
            );

            let mut dc = o.get_draw_call(&mut program);
            dc.set_mat3("m_inv", &m_inv);
            dc.set_mat3("mv_inv", &mv_inv);
            dc.set_mat4("model", &model_isom);
            dc.set_mat4("view", &camera.view());
            dc.set_mat4("projection", &camera.projection);
            dc.set_int("diffuseMap", 0);

            dc.perform();
        }*/
    });
}
