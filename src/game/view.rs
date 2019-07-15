use cgmath::{Matrix4, Matrix3, Point3, Vector3};

pub struct Camera {
    position: Point3<f32>,
    look_direction: Vector3<f32>,
    interpolated_step: Option<Vector3<f32>>,
    move_direction: Option<Vector3<f32>>,
    up: Vector3<f32>,
    speed: f32,
}

pub fn new_camera(position: Point3<f32>, look_direction: Vector3<f32>) -> Camera {
    let up = Vector3::<f32>::unit_y();
    Camera {
        position,
        look_direction,
        interpolated_step: None,
        move_direction: None,
        up,
        speed: 1.0,
    }
}

impl Camera {
    pub fn get_int_view_matrix(&mut self, i_v: &f32) -> Matrix4<f32> {
        match self.move_direction {
            Some(m_d) => {
                let is = m_d * *i_v ;
                self.interpolated_step = Some(is.clone());
                Matrix4::look_at_dir(self.position+is, self.look_direction, self.up)
            }
            None => {
                Matrix4::look_at_dir(self.position, self.look_direction, self.up)
            }
        }
    }

    pub fn move_dir(&mut self, dir: Option<Vector3<f32>>) {

        self.move_direction = dir;

        match self.interpolated_step {
            Some(is) => {
                self.position+=is;
                self.interpolated_step = None;
            },
            None =>()
        }
        
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let m = Matrix3::from_angle_x(cgmath::Rad(angle));
        self.look_direction = m * self.look_direction;
    }
    pub fn rotate_y(&mut self, angle: f32) {
        let m = Matrix3::from_angle_y(cgmath::Rad(angle));
        self.look_direction = m * self.look_direction;
    }

    pub fn get_dir(&self) -> Vector3<f32> { self.look_direction }
    pub fn get_speed(&self) -> f32 { self.speed }

    /*
    pub fn starts_moving(&mut self){self.moving=true}
    pub fn stops_moving(&mut self){self.moving=false}
    pub fn is_moving(&mut self) -> bool {self.moving}
    */
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position \n {:?} \n direction {:?}", self.position, self.look_direction)
    }
}