use glm::*;

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub pitch: f32,
    pub yaw: f32,

    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,

    pub projection: Mat4,
}

impl Camera {
    pub fn new_perspective(aspect: f32, fov: f32, near: f32, far: f32, pos: Vec3) -> Camera {
        Camera {
            projection: perspective(aspect, fov, near, far),

            aspect: aspect,
            fov: fov,
            near: near,
            far: far,

            pos: pos,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.projection = perspective(self.aspect, self.fov, self.near, self.far);
    }

    pub fn rotation(&self) -> Quat {
        let ret = quat_angle_axis(self.pitch, &vec3(1.0, 0.0, 0.0))
            * quat_angle_axis(self.yaw, &vec3(0.0, 1.0, 0.0));
        quat_normalize(&ret)
    }

    pub fn move_dir(&mut self, dir: Vec3) {
        let scaled_vec = dir;
        let rotated = quat_rotate_vec3(&self.rotation(), &scaled_vec);
        self.pos += rotated;
    }

    pub fn move_forward(&mut self, d: f32) {
        self.move_dir(vec3(0.0, 0.0, d));
    }

    pub fn move_right(&mut self, d: f32) {
        self.move_dir(vec3(d, 0.0, 0.0));
    }

    pub fn move_up(&mut self, d: f32) {
        self.move_dir(vec3(0.0, d, 0.0));
    }

    pub fn view(&self) -> Mat4 {
        quat_cast(&self.rotation())
    }

    pub fn camera_matrix(&self) -> Mat4 {
        self.projection * self.view()
    }
}

pub struct OrthoCamera {
    pub projection: Mat4,
}

/*
Matrix4::new(
                2.0/w,    0.0,  0.0, 0.0,
                0.0,   2.0/-h,  0.0, 0.0,
                0.0,      0.0, -1.0, 0.0,
                -1.0,      1.0,  0.0, 1.0,
            )
*/
impl OrthoCamera {
    pub fn new(w: f32, h: f32) -> OrthoCamera {
        let o = ortho(0.0, w, 0.0, h, -10.0, 1000.0);
        let s = scaling(&vec3(1.0, -1.0, 1.0));

        OrthoCamera { projection: s * o }
    }
}
