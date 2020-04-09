use cgmath::{Rad, Matrix3};
use std::f32::consts::PI;

pub const TWO_PI: Rad<f32> = Rad(2.0 * PI);
lazy_static!{
    pub static ref HALF_PI_ROTATION_MATRIX_LEFT:Matrix3<f32> = Matrix3::from_angle_y(Rad(std::f32::consts::PI / 2.0));
}