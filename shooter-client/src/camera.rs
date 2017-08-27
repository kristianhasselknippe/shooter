use super::na::*;

pub struct Camera {
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new_orthographic(w: f32, h: f32) -> Camera {
        let w = w/2.0;
        let h = h/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            projection: proj,
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}
