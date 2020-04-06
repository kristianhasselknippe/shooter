#![allow(unused_imports)]
extern crate image as img;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate imgui_sys as imgui;
extern crate itertools;
extern crate nalgebra_glm as glm;
extern crate ncollide3d as nc;
extern crate num_traits;
extern crate ordered_float as of;
extern crate rusttype;
extern crate specs;
extern crate specs_derive;
extern crate time as t;
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

pub mod camera;
pub mod drawing;
pub mod fps_counter;
pub mod image;
pub mod mesh;
pub mod shader;
pub mod time;
pub mod transform;
pub mod utils;
pub mod window;

use camera::*;
use drawing::*;
use glm::*;
use shader::*;
use specs::prelude::*;
use time::*;
use window::init_vulkano_window;

use winit::{
	event::WindowEvent,
	event_loop::ControlFlow,
};

fn viewport(_wl: i32, _h: i32) {
    unimplemented!();
}

fn clear(_a: f32, _b: f32, _c: f32, _d: f32) {
    unimplemented!();
}

pub fn start_event_loop() {
    let mut window_size = (800, 600);
    let (mut events_loop, gl_context) = init_vulkano_window(window_size);

	events_loop.run(|event, _event_loop_window_target, control_flow| {
		match event {
			winit::event::Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
				*control_flow = winit::event_loop::ControlFlow::Exit
			},
			_ => (),
		}
	});
}
