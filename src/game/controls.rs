use cgmath::{Matrix3, Rad, Vector3};
use cgmath::prelude::InnerSpace;

pub struct Controls {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub fn new_controls() -> Controls {
    Controls {
        forward: false,
        backward: false,
        left: false,
        right: false,
        up: false,
        down: false,
    }
}

impl Controls {
    pub fn reset(&mut self) {
        self.forward = false;
        self.backward = false;
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
    }
    pub fn get_movement_vec(&self, look_dir: Vector3<f32>, speed: f32) -> Option<Vector3<f32>> {
        let mut v = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        use crate::common_consts::HALF_PI_ROTATION_MATRIX_LEFT;

        let mut norm = look_dir;
        norm.y = 0.0;
        norm = norm.normalize();
        norm = *HALF_PI_ROTATION_MATRIX_LEFT * norm;

        if self.forward {
            v += look_dir;
        }
        if self.backward {
            v -= look_dir;
        }
        if self.left {
            v += norm;
        }
        if self.right {
            v -= norm;
        }
        if self.up {
            v += Vector3::unit_y();
        }
        if self.down {
            v -= Vector3::unit_y();
        }

        if v.x != 0.0 || v.y != 0.0 || v.z != 0.0 {
            return Some(v.normalize_to(speed));
        }
        None
    }
}
