#![allow(dead_code)]

extern crate alga;
extern crate engine;
extern crate gl;
extern crate glutin;
extern crate nalgebra_glm as glm;
extern crate specs;
extern crate specs_derive;

use engine::start_event_loop;
use glm::*;
use glutin::{ContextBuilder, EventsLoop, GlContext, GlWindow, WindowBuilder};

fn main() {
    start_event_loop();
}
