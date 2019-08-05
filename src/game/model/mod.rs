pub mod mesh;
pub mod motion;

mod rotation_utility;

use cgmath::{Matrix4, Rad, Vector3};

use crate::settings::TICKS_PER_SECOND;

pub struct Model {
    mesh: mesh::Mesh,
    rotation_x: Rad<f32>,
    rotation_y: Rad<f32>,
    rotation_z: Rad<f32>,
    rotation_matrix: Option<Matrix4<f32>>,
    translation: Vector3<f32>,
    translation_matrix: Option<Matrix4<f32>>,
    scale: f32,
    scale_matrix: Option<Matrix4<f32>>,
    model_matrix: Option<Matrix4<f32>>,
    motion: motion::Motion,
    texture: gl::types::GLuint,
}

impl Model {
    pub fn draw_3d(&self) {
        self.mesh.draw_3d();
    }

    pub fn cleanup(&self) {
        self.mesh.cleanup();
        unsafe {
            gl::DeleteTextures(1, &self.texture);
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation_x += Rad(angle);
    }
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation_x += Rad(angle);
    }
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation_x += Rad(angle);
    }

    pub fn set_rotation_speed_x(&mut self, angle: f32) {
        self.motion.set_rotation_speed_x(angle);
    }
    pub fn set_rotation_speed_y(&mut self, angle: f32) {
        self.motion.set_rotation_speed_y(angle);
    }
    pub fn set_rotation_speed_z(&mut self, angle: f32) {
        self.motion.set_rotation_speed_z(angle);
    }

    pub fn set_movement_vector(&mut self, vec: Vector3<f32>) {
        self.motion.movement_vector = Some(vec);
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale *= scale;
        self.scale_matrix = None;
    }
    pub fn translate(&mut self, translate: Vector3<f32>) {
        self.translation += translate;
    }

    pub fn update(&mut self) {
        self.advance_rotation();
        self.advance_translation();
        self.trim_angel();
    }

    fn trim_angel(&mut self) {
        let two_pi = Rad(2.0 * PI);
        if self.rotation_x > two_pi {
            self.rotation_x -= Rad(2.0 * PI)
        }
        if self.rotation_y > two_pi {
            self.rotation_y -= Rad(2.0 * PI)
        }
        if self.rotation_z > two_pi {
            self.rotation_z -= Rad(2.0 * PI)
        }
    }

    fn advance_translation(&mut self) {
        use crate::settings::TICKS_PER_SECOND as tps;

        let mut change_occurred = false;
        self.motion.movement_vector.map(|v| {
            self.translation += v / tps;
            change_occurred = true;
        });

        if change_occurred {
            self.translation_matrix = None;
            self.model_matrix = None;
        }
    }

    fn advance_rotation(&mut self) {
        use crate::settings::TICKS_PER_SECOND as tps;

        let mut change_occurred = false;
        self.motion.x_rotation_speed.map(|a| {
            self.rotation_x += a / tps;
            change_occurred = true;
        });
        self.motion.y_rotation_speed.map(|a| {
            self.rotation_y += a / tps;
            change_occurred = true;
        });
        self.motion.z_rotation_speed.map(|a| {
            self.rotation_z += a / tps;
            change_occurred = true;
        });

        if change_occurred {
            self.rotation_matrix = None;
            self.model_matrix = None;
        }
    }

    pub fn get_model_matrix(&mut self) -> Matrix4<f32> {
        match self.model_matrix {
            Some(m) => m,
            None => {
                let rotation = self.get_rotation_matrix();
                let translation = self.get_translation_matrix();
                let scale = self.get_scale_matrix();

                let m_m = translation * rotation * scale;

                self.model_matrix = Some(m_m);
                m_m
            }
        }
    }

    pub fn get_intr_model_matrix(&mut self, i_v: f32) -> Matrix4<f32> {

        let rotation = self.get_intr_rotation_matrix(i_v);
        let translation = self.get_intr_translation_matrix(i_v);
        let scale = self.get_scale_matrix();

        let m_m = translation * rotation * scale;
        m_m
    }

    fn get_intr_rotation_matrix(&mut self, i_v: f32) -> Matrix4<f32> {
        use crate::settings::TICKS_PER_SECOND as tps;
        if self.motion.is_rotating() {
            let r_x = cgmath::Matrix4::from_angle_x(self.rotation_x + self.motion.get_intr_rotation_speed_x(i_v)/tps);
            let r_y = cgmath::Matrix4::from_angle_y(self.rotation_y + self.motion.get_intr_rotation_speed_y(i_v)/tps);
            let r_z = cgmath::Matrix4::from_angle_z(self.rotation_z + self.motion.get_intr_rotation_speed_z(i_v)/tps);
            r_z * r_y * r_x
        } else {
            self.get_rotation_matrix()
        }
    }

    fn get_rotation_matrix(&mut self) -> Matrix4<f32> {
        match self.rotation_matrix {
            Some(m) => m,
            None => {
                let r_x = cgmath::Matrix4::from_angle_x(self.rotation_x);
                let r_y = cgmath::Matrix4::from_angle_y(self.rotation_y);
                let r_z = cgmath::Matrix4::from_angle_z(self.rotation_z);
                let r_m = r_z * r_y * r_x;
                self.rotation_matrix = Some(r_m);
                r_m
            }
        }
    }

    fn get_scale_matrix(&mut self) -> Matrix4<f32> {
        match self.scale_matrix {
            Some(m) => m,
            None => {
                let s = cgmath::Matrix4::from_scale(self.scale);
                self.scale_matrix = Some(s);
                s
            }
        }
    }

    fn get_intr_translation_matrix(&mut self, i_v: f32) -> Matrix4<f32> {
        use crate::settings::TICKS_PER_SECOND as tps;
        if self.motion.is_moving() {
            return cgmath::Matrix4::from_translation(self.translation + self.motion.get_intr_movement(i_v)/tps);
        }
        self.get_translation_matrix()
    }

    fn get_translation_matrix(&mut self) -> Matrix4<f32> {
        match self.translation_matrix {
            Some(m) => m,
            None => {
                let t_m = cgmath::Matrix4::from_translation(self.translation);
                self.translation_matrix = Some(t_m);
                t_m
            }
        }
    }

    pub fn get_translation(&self) -> Vector3<f32> {
        self.translation
    }

    pub fn get_texture(&self) -> gl::types::GLuint {
        self.texture
    }
}

mod example;

pub fn triangle() -> Model {
    let mesh = crate::game::loader::load_collada_mesh(include_str!("res/untitled.dae"));

    Model {
        mesh,
        translation: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation_x: Rad(-PI / 2.0),
        rotation_y: Rad(0.0),
        rotation_z: Rad(0.0),
        scale: 1.0,
        scale_matrix: None,
        model_matrix: None,
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png"),
        rotation_matrix: None,
        translation_matrix: None,
    }
}

pub fn plane() -> Model {
    let mesh = crate::game::loader::load_collada_mesh(include_str!("res/plane.dae"));

    Model {
        mesh,
        translation: Vector3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        },
        rotation_x: Rad(0.0),
        rotation_y: Rad(0.0),
        rotation_z: Rad(0.0),
        scale: 1.0,
        scale_matrix: None,
        model_matrix: None,
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png"),
        rotation_matrix: None,
        translation_matrix: None,
    }
}

pub fn dist_cube() -> Model {
    let mesh = crate::game::loader::load_collada_mesh(include_str!("res/dist.dae"));

    Model {
        mesh,
        translation: Vector3 {
            x: -5.0,
            y: 0.0,
            z: 0.0,
        },
        rotation_x: Rad(0.0),
        rotation_y: Rad(0.0),
        rotation_z: Rad(0.0),
        scale: 1.0,
        scale_matrix: None,
        model_matrix: None,
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png"),
        rotation_matrix: None,
        translation_matrix: None,
    }
}

use gl::types::*;
use std::f32::consts::PI;

static VERTEX_DATA_2D: [GLfloat; 12] = [0.25, 1.0, 0.0, 1.0, 1.0, 0.0, 0.25, 0.25, 0.0, 1.0, 0.25, 0.0];
static UV_DATA_2D: [GLfloat; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
static NORMAL_DATA_2D: [GLfloat; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
static INDICES: [i16; 6] = [0, 1, 2, 2, 1, 3];

pub fn io_2d() -> Model {
    Model {
        mesh: mesh::new_static_3d_mesh(VERTEX_DATA_2D.to_vec(), NORMAL_DATA_2D.to_vec(), UV_DATA_2D.to_vec(), INDICES.to_vec()),
        translation: Vector3 {
            x: -5.0,
            y: 1.0,
            z: 0.0,
        },
        rotation_x: Rad(0.0),
        rotation_y: Rad(0.0),
        rotation_z: Rad(0.0),
        scale: 1.0,
        scale_matrix: None,
        model_matrix: None,
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png"),
        rotation_matrix: None,
        translation_matrix: None,
    }
}