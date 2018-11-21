use na::{Vector2,Rotation2,Point2};

pub struct Transform {
    position: Point2<f64>,
    rotation: Rotation2<f64>,
    scale: Vector2<f64>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Point2::new(0.0,0.0),
            rotation: Rotation2::new(0.0),
            scale: Vector2::new(1.0,1.0),
        }
    }
}
