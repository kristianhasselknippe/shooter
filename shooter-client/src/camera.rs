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
        let w = width / 2.0;
        let h = height / 2.0;
        let proj = Matrix4::new_orthographic(-w, w, -h, h, 0.0, 1000.0);
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
            yaw: -3.14/2.0,
        }
    }

    pub fn move_forward(&mut self, d: f32) {
        self.pos -= self.direction() * d;
    }

    pub fn move_right(&mut self, d: f32) {
        let origDir = self.direction();
        let dir = Vector3::new(origDir.x, 0.0, origDir.z);
        let dir = Rotation3::from_euler_angles(0.0, 3.14/2.0, 0.0) * dir;
        self.pos -= dir * d;
    }

    pub fn move_up(&mut self, d: f32)  {
        self.pos += Vector3::new(0.0,d,0.0);
    }

    pub fn direction(&self) -> Vector3<f32> {
        Vector3::new(self.pitch.cos() * self.yaw.cos(),
                     self.pitch.sin(),
                     self.pitch.cos() * self.yaw.sin()).normalize()
    }

    pub fn view(&self) -> Matrix4<f32> {
        let target = self.pos + self.direction();
        Isometry3::look_at_rh(&self.pos, &target, &Vector3::y()).to_homogeneous()
    }

    pub fn camera_matrix(&self) -> Matrix4<f32> {
        self.projection * self.view()
    }
}
