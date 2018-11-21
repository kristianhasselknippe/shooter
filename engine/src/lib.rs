extern crate alga;
extern crate gl;
extern crate glutin;
extern crate image as img;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate imgui_sys as imgui;
extern crate itertools;
extern crate nalgebra as na;
extern crate ncollide3d as nc;
extern crate ordered_float as of;
extern crate rusttype;
extern crate time as t;

extern crate specs;
extern crate specs_derive;

pub mod drawing;
pub mod image;
pub mod mesh;
pub mod scene;
pub mod shader;
pub mod transform;
pub mod utils;
// mod text;
pub mod camera;
pub mod fps_counter;
pub mod gui;
pub mod input;
pub mod time;

#[test]
fn test() {
    println!("We had a test");
}
