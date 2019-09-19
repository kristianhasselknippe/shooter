#![allow(dead_code)]
use glm::*;

pub struct Transform {
    position: Vec2,
    rotation: Vec2,
    scale: Vec2,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: vec2(0.0, 0.0),
            rotation: vec2(0.0, 0.0),
            scale: vec2(1.0, 1.0),
        }
    }
}
