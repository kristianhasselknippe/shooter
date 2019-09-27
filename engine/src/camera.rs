use glm::*;
use num_traits::*;

#[derive(Debug)]
pub struct Camera<T: Scalar + FromPrimitive> {
    pub pos: TVec3<T>,
    pub pitch: T,
    pub yaw: T,

    pub aspect: T,
    pub fov: T,
    pub near: T,
    pub far: T,

    pub projection: TMat4<T>,
}

impl<T: RealField + FromPrimitive> Camera<T> {
    pub fn new_perspective(aspect: T, fov: T, near: T, far: T, pos: TVec3<T>) -> Camera<T> {
        Camera {
            projection: perspective(aspect, fov, near, far),

            aspect: aspect,
            fov: fov,
            near: near,
            far: far,

            pos: pos,
            pitch: T::from_f64(0.0).unwrap(),
            yaw: T::from_f64(0.0).unwrap(),
        }
    }

    pub fn set_aspect(&mut self, aspect: T) {
        self.aspect = aspect;
        self.projection = perspective(self.aspect, self.fov, self.near, self.far);
    }

    pub fn rotation(&self) -> Qua<T> {
        let ret = quat_angle_axis(
            self.pitch,
            &vec3(
                T::from_f64(1.0).unwrap(),
                T::from_f64(0.0).unwrap(),
                T::from_f64(0.0).unwrap(),
            ),
        ) * quat_angle_axis(
            self.yaw,
            &vec3(
                T::from_f64(0.0).unwrap(),
                T::from_f64(1.0).unwrap(),
                T::from_f64(0.0).unwrap(),
            ),
        );
        quat_normalize(&ret)
    }

    pub fn move_dir(&mut self, dir: TVec3<T>) {
        let scaled_vec = dir;
        let rotated = quat_rotate_vec3(&self.rotation(), &scaled_vec);
        self.pos += rotated;
    }

    pub fn move_forward(&mut self, d: T) {
        self.move_dir(vec3(
            T::from_f64(0.0).unwrap(),
            T::from_f64(0.0).unwrap(),
            d,
        ));
    }

    pub fn move_right(&mut self, d: T) {
        self.move_dir(vec3(
            d,
            T::from_f64(0.0).unwrap(),
            T::from_f64(0.0).unwrap(),
        ));
    }

    pub fn move_up(&mut self, d: T) {
        self.move_dir(vec3(
            T::from_f64(0.0).unwrap(),
            d,
            T::from_f64(0.0).unwrap(),
        ));
    }

    pub fn view(&self) -> TMat4<T> {
        quat_cast(&self.rotation())
    }

    pub fn camera_matrix(&self) -> TMat4<T> {
        self.projection * self.view()
    }
}
