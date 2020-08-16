use cgmath::{Rad, Vector3, Quaternion, Rotation3, One};

pub enum MotionType{
    continuous(ContinuousMotion),
    q_to_q(Q_to_QMotion)
}
pub struct Q_to_QMotion {
    pub last_quaternion: Quaternion<f32>,
    pub target_quaternion: Quaternion<f32>,
}

pub struct ContinuousMotion {
    // Units/sec
    pub movement_vector: Vector3<f32>,
    // Degree/sec
    pub rotation_per_sec: Quaternion<f32>,
}

pub fn new_q_to_q_motion() -> Q_to_QMotion {
    Q_to_QMotion {
        last_quaternion: Quaternion::one(),
        target_quaternion: Quaternion::one()
    }
}

pub fn new_continuous_motion() -> ContinuousMotion {
    ContinuousMotion {
        movement_vector: Vector3::new(0.0,0.0,0.0),
        rotation_per_sec: Quaternion::one()
    }
}

use crate::settings::TICKS_PER_SECOND as tps;

impl ContinuousMotion {

    pub fn set_rotation_per_sec_x(&mut self, x_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_x(), x_rs/tps);
    }
    pub fn set_rotation_per_sec_y(&mut self, y_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_y(), y_rs/tps);
    }
    pub fn set_rotation_per_sec_z(&mut self, z_rs: Rad<f32>) {
        self.rotation_per_sec = Quaternion::from_axis_angle(Vector3::unit_z(), z_rs/tps);
    }
}
