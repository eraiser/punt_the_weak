use cgmath::{Rad, Vector3, Quaternion, Rotation3, One};

pub struct Motion {
    // Units/sec
    pub movement_vector: Vector3<f32>,
    // Degree/sec
    pub rotation_per_sec: Quaternion<f32>,
    pub target_quaternion: Quaternion<f32>
}

pub fn new_motion() -> Motion {
    Motion {
        movement_vector: Vector3::new(0.0,0.0,0.0),
        rotation_per_sec: Quaternion::one(),
        target_quaternion: Quaternion::one()
    }
}

impl Motion {

    pub fn set_rotation_per_sec_x(&mut self, x_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_x(), x_rs);
    }
    pub fn set_rotation_per_sec_y(&mut self, y_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_y(), y_rs);
    }
    pub fn set_rotation_per_sec_z(&mut self, z_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_z(), z_rs);
    }
}
