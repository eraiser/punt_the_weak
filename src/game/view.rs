use cgmath::{Rad,Matrix4, Matrix3, Point3, Vector3};

pub struct Camera {
    position: Point3<f32>,
    look_direction: Vector3<f32>,
    current_direction: Option<Vector3<f32>>,
    x_rotation: Rad<f32>,
    y_rotation: Rad<f32>,
    rotation_matrix: Option<Matrix3<f32>>,
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
        current_direction: None,
        x_rotation: Rad(0.0),
        y_rotation: Rad(0.0),
        rotation_matrix: None,
        interpolated_step: None,
        move_direction: None,
        up,
        speed: 2.0,
    }
}

impl Camera {
    pub fn get_int_view_matrix(&mut self, i_v: &f32) -> Matrix4<f32> {
        match self.move_direction {
            Some(m_d) => {
                let is = m_d * *i_v ;
                self.interpolated_step = Some(is.clone());

                Matrix4::look_at_dir(self.position+is, self.get_rotation()*self.look_direction, self.up)
            }
            None => {
                Matrix4::look_at_dir(self.position, self.get_rotation()*self.look_direction, self.up)
            }
        }
    }

    pub fn move_dir(&mut self, dir: Option<Vector3<f32>>) {

        if let Some(d) = self.move_direction {
            self.position += d;
        }

        self.move_direction = dir;

    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.x_rotation+=Rad(angle);
        self.rotation_matrix = None;
        self.current_direction = None;
    }
    pub fn rotate_y(&mut self, angle: f32) {
        self.y_rotation+=Rad(angle);
        self.rotation_matrix = None;
        self.current_direction = None;
    }

    fn get_rotation(&mut self) -> Matrix3<f32> {
        match self.rotation_matrix {
            Some(m) => return m,
            None => {
                let m = Matrix3::from_angle_y(self.y_rotation)*
                        Matrix3::from_angle_x(self.x_rotation);
                self.rotation_matrix= Some(m);
                return m;
            }
        }
    }

    pub fn get_current_dir(&mut self) -> Vector3<f32> {
        match self.current_direction {
            Some(v) => return v,
            None => {
                let v = self.get_rotation()*self.look_direction;
                self.current_direction = Some(v);
                return v;
            }
        }
    }
    pub fn get_speed(&self) -> f32 { self.speed }
    pub fn set_speed(&mut self,s:f32) { self.speed = s}
    pub fn get_position(&self) -> Point3<f32> {self.position}

}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position \n {:?} \n direction {:?}", self.position, self.look_direction)
    }
}