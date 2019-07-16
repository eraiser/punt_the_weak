

use cgmath::{Matrix4, Rad};
use cgmath::prelude::SquareMatrix;

use crate::settings::TICKS_PER_SECOND;

pub fn generate_rotation_matrix(x_rotation_speed: Option<Rad<f32>>,
                                y_rotation_speed: Option<Rad<f32>>,
                                z_rotation_speed: Option<Rad<f32>>) -> Matrix4<f32>{
    let tps = TICKS_PER_SECOND as f32;
    match z_rotation_speed {
        Some(z_r) => {
            let m_z: Matrix4<f32> = Matrix4::from_angle_z(z_r/tps);
            match y_rotation_speed {
                Some(y_r) => {
                    let m_y: Matrix4<f32> = Matrix4::from_angle_y(y_r/tps);
                    match x_rotation_speed {
                        Some(x_r) => {
                            let m_x: Matrix4<f32> = Matrix4::from_angle_x(x_r/tps);
                            return m_z * m_y * m_x;
                        }
                        None => return m_z * m_y
                    }
                }
                None => match x_rotation_speed {
                    Some(x_r) => {
                        let m_x: Matrix4<f32> = Matrix4::from_angle_x(x_r/tps);
                        return m_z * m_x;
                    }
                    None => return m_z
                }
            }
        }
        None => match y_rotation_speed {
            Some(y_r) => {
                let m_y: Matrix4<f32> = Matrix4::from_angle_y(y_r/tps);
                match x_rotation_speed {
                    Some(x_r) => {
                        let m_x: Matrix4<f32> = Matrix4::from_angle_x(x_r/tps);
                        return m_y * m_x;
                    }
                    None => return m_y
                }
            }
            None => match x_rotation_speed {
                Some(x_r) => {
                    let m_x: Matrix4<f32> = Matrix4::from_angle_x(x_r/tps);
                    return m_x;
                }
                None => return Matrix4::identity()
            }
        }
    }
}

pub fn generate_int_rotation_matrix(x_rotation_speed: Option<Rad<f32>>,
                                y_rotation_speed: Option<Rad<f32>>,
                                z_rotation_speed: Option<Rad<f32>>,
                                    i_v: &f32) -> Matrix4<f32>{
    let tps = TICKS_PER_SECOND as f32;
    match z_rotation_speed {
        Some(z_r) => {
            let m_z: Matrix4<f32> = Matrix4::from_angle_z((z_r/tps)* *i_v);
            match y_rotation_speed {
                Some(y_r) => {
                    let m_y: Matrix4<f32> = Matrix4::from_angle_y((y_r/tps)* *i_v);
                    match x_rotation_speed {
                        Some(x_r) => {
                            let m_x: Matrix4<f32> = Matrix4::from_angle_x((x_r/tps)* *i_v);
                            return m_z * m_y * m_x;
                        }
                        None => return m_z * m_y
                    }
                }
                None => match x_rotation_speed {
                    Some(x_r) => {
                        let m_x: Matrix4<f32> = Matrix4::from_angle_x((x_r/tps)* *i_v);
                        return m_z * m_x;
                    }
                    None => return m_z
                }
            }
        }
        None => match y_rotation_speed {
            Some(y_r) => {
                let m_y: Matrix4<f32> = Matrix4::from_angle_y((y_r/tps)* *i_v);
                match x_rotation_speed {
                    Some(x_r) => {
                        let m_x: Matrix4<f32> = Matrix4::from_angle_x((x_r/tps)* *i_v);
                        return m_y * m_x;
                    }
                    None => return m_y
                }
            }
            None => match x_rotation_speed {
                Some(x_r) => {
                    let m_x: Matrix4<f32> = Matrix4::from_angle_x((x_r/tps)* *i_v);
                    return m_x;
                }
                None => return Matrix4::identity()
            }
        }
    }
}