use super::na::*;

#[derive(Debug)]
pub struct Camera {
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new_orthographic(width: f32, height: f32) -> Camera {
        let w = width/2.0;
        let h = height/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            projection: proj,
        }
    }

    pub fn new_perspective(aspect: f32, near: f32, far: f32) -> Camera {
        Camera {
            projection: na::Perspective3::new(aspect, near, far),
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}
