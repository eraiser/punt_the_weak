
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

impl Motion {

    pub fn get_intr_rotation_speed_x(&mut self, i_v: f32) -> Option<Rad<f32>> {
        self.x_rotation_speed.map(  |r| r*i_v)
    }
    pub fn get_intr_rotation_speed_y(&mut self, i_v: f32) -> Option<Rad<f32>> {
        self.y_rotation_speed.map(  |r| r*i_v)
    }
    pub fn get_intr_rotation_speed_z(&mut self, i_v: f32) -> Option<Rad<f32>> {
        self.z_rotation_speed.map(  |r| r*i_v)
    }
    pub fn get_intr_movement(&mut self, i_v: f32) -> Vector3<f32> {
        use cgmath::prelude::Zero;
        self.movement_vector.map_or_else( || cgmath::Vector3::zero(), |r| r*i_v)
    }

    pub fn set_rotation_speed_x(&mut self, x_rs: f32) {
        if x_rs != 0.0 {
            self.x_rotation_speed = Some(Rad(x_rs));
        } else {
            self.x_rotation_speed = None;
        }
    }
    pub fn set_rotation_speed_y(&mut self, y_rs: f32) {
        if y_rs != 0.0 {
            self.y_rotation_speed = Some(Rad(y_rs));
        } else {
            self.y_rotation_speed = None;
        }
    }
    pub fn set_rotation_speed_z(&mut self, z_rs: f32) {
        if z_rs != 0.0 {
            self.z_rotation_speed = Some(Rad(z_rs));
        } else {
            self.z_rotation_speed = None;
        }
    }
    pub fn is_rotating(&self) -> bool {
        if self.x_rotation_speed.is_some() || self.y_rotation_speed.is_some() || self.z_rotation_speed.is_some() {
            true
        }else {
            false
        }
    }
    pub fn is_moving(&self) -> bool {
        self.movement_vector.is_some()
    }
}
