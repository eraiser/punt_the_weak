
use cgmath::{Rad, Vector3};
pub struct Motion {
    pub movement_vector: Option<Vector3<f32>>,
    // Units/sec
    pub x_rotation_speed: Option<Rad<f32>>,
    pub y_rotation_speed: Option<Rad<f32>>,
    pub z_rotation_speed: Option<Rad<f32>>,
    // Degree/sec
}

pub fn new_motion() -> Motion {
    Motion {
        movement_vector:    None,
        x_rotation_speed:   None,
        y_rotation_speed:   None,
        z_rotation_speed:   None,
    }
}

