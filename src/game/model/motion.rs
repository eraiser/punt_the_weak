pub struct Motion {
    pub x_movement_speed: f32,
    pub y_movement_speed: f32,
    pub z_movement_speed: f32,
    // Units/sec
    pub x_rotation_speed: f32,
    pub y_rotation_speed: f32,
    pub z_rotation_speed: f32,
    // Revolutions/sec
}

pub fn new_motion() -> Motion {
    Motion {
        x_movement_speed: 0.0,
        y_movement_speed: 0.0,
        z_movement_speed: 0.0,
        x_rotation_speed: 0.0,
        y_rotation_speed: 0.0,
        z_rotation_speed: 0.0,
    }
}

