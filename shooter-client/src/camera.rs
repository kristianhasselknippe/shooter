use super::na::*;

#[derive(Debug)]
pub struct Camera {
    pub pos: Point3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub projection: Matrix4<f32>,
}

impl Camera {
    pub fn new_orthographic(width: f32, height: f32, pos: Point3<f32>) -> Camera {
        let proj = Matrix4::new_orthographic(0.0, width, 0.0, height, -10.0, 1000.0);
        Camera {
            projection: proj,
            pos: pos,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn new_perspective(aspect: f32, fov: f32, near: f32, far: f32, pos: Point3<f32>) -> Camera {
        Camera {
            projection: ::na::Perspective3::new(aspect, fov, near, far).as_matrix().clone(),
            pos: pos,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn rotation(&self) -> Rotation3<f32> {
        Rotation3::from_axis_angle(&Vector3::x_axis(), self.pitch) *
        Rotation3::from_axis_angle(&Vector3::y_axis(), self.yaw)
    }

    pub fn move_forward(&mut self, d: f32) {
        self.pos += self.rotation().inverse() * (Vector3::z() * d);
    }

    pub fn move_right(&mut self, d: f32) {
        self.pos += self.rotation().inverse() * (Vector3::x() * d);
    }

    pub fn move_up(&mut self, d: f32)  {
        self.pos += self.rotation().inverse() * (Vector3::y() * d);
    }

    pub fn view(&self) -> Matrix4<f32> {
        let iso = self.rotation() * Translation3::from_vector(Vector3::new(-self.pos.x, -self.pos.y, -self.pos.z));
        iso.to_homogeneous()
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection * self.view()
    }
}
