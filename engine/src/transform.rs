use glm::*;

pub struct Transform {
    position: DVec2,
    rotation: DVec2,
    scale: DVec2,
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
