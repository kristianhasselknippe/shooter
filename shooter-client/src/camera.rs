use super::na::*;

#[derive(Debug)]
pub struct Camera {
    pub projection: Matrix4<f32>,
    pub size: (f32,f32)
}

impl Camera {
    pub fn new_orthographic(width: f32, height: f32) -> Camera {
        let w = width/2.0;
        let h = height/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            projection: proj,
            size: (width,height),
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}
