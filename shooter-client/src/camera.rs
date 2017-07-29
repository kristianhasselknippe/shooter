use super::na::*;

pub struct Camera {
    translation: Vector3<f32>,
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new_orthographic(w: f32, h: f32, pos: Vector3<f32>) -> Camera {
        let w = w/2.0;
        let h = h/2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h,0.0,1000.0);
        Camera {
            translation: pos,
            projection: proj,
        }
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection.append_translation(&self.translation)
    }

    pub fn translate(&mut self, t: Vector3<f32>) {
        self.translation.x += t.x;
        self.translation.y += t.y;
        self.translation.z += t.z;
    }
}
