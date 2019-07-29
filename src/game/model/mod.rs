
pub mod mesh;
pub mod motion;

mod rotation_utility;

use cgmath::{Matrix4, Rad, Vector3};

use crate::settings::TICKS_PER_SECOND;

pub struct Model {
    mesh: mesh::Mesh,
    rotation: Matrix4<f32>,
    translation: Vector3<f32>,
    scale: f32,
    model_matrix: Matrix4<f32>,
    motion: motion::Motion,
    texture: gl::types::GLuint,
}

impl Model {
    pub fn draw_3d(&self) {
        self.mesh.draw_3d();
    }

    pub fn cleanup(&self) {
        self.mesh.cleanup();
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let m = Matrix4::from_angle_x(Rad(angle));
        self.rotation = self.rotation * m;
    }
    pub fn rotate_y(&mut self, angle: f32) {
        let m = Matrix4::from_angle_y(Rad(angle));
        self.rotation = self.rotation * m;
    }
    pub fn rotate_z(&mut self, angle: f32) {
        let m = Matrix4::from_angle_z(Rad(angle));
        self.rotation = self.rotation * m;
    }

    pub fn set_rotation_speed_x(&mut self, angle: f32) {
        self.motion.x_rotation_speed = Some(Rad(angle));
    }
    pub fn set_rotation_speed_y(&mut self, angle: f32) {
        self.motion.y_rotation_speed = Some(Rad(angle));
    }
    pub fn set_rotation_speed_z(&mut self, angle: f32) {
        self.motion.z_rotation_speed = Some(Rad(angle));
    }

    pub fn set_movement_vector(&mut self, vec: Vector3<f32>) {
        self.motion.movement_vector = Some(vec);
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale *= scale;
    }
    pub fn translate(&mut self, translate: Vector3<f32>) {
        self.translation += translate;
    }

    pub fn update(&mut self) {
        let rotation: Matrix4<f32> = rotation_utility::generate_rotation_matrix(
            self.motion.x_rotation_speed,
            self.motion.y_rotation_speed,
            self.motion.z_rotation_speed,
        );

        self.rotation = self.rotation * rotation;

        if let Some(v) = self.motion.movement_vector {
            self.translation += v / TICKS_PER_SECOND as f32;
        }
        self.model_matrix =
            Matrix4::from_translation(self.translation) * self.rotation * self.scale;
    }
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }
    pub fn get_int_model_matrix(&self, i_v: &f32) -> Matrix4<f32> {
        let rotation: Matrix4<f32> = rotation_utility::generate_int_rotation_matrix(
            self.motion.x_rotation_speed,
            self.motion.y_rotation_speed,
            self.motion.z_rotation_speed,
            i_v,
        );

        if let Some(v) = self.motion.movement_vector {
            return Matrix4::from_translation(
                self.translation + ((v / TICKS_PER_SECOND as f32) * *i_v),
            ) * self.rotation
                * rotation
                * self.scale;
        }
        Matrix4::from_translation(self.translation) * self.rotation * rotation * self.scale
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
        rotation: cgmath::SquareMatrix::identity(),
        scale: 1.0,
        model_matrix: cgmath::SquareMatrix::identity(),
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png")
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
        rotation: cgmath::SquareMatrix::identity(),
        scale: 1.0,
        model_matrix: cgmath::SquareMatrix::identity(),
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png")
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
        //rotation: cgmath::SquareMatrix::identity(),
        rotation: cgmath::Matrix4::from_angle_x(Rad(-std::f32::consts::PI/2.0)),
        scale: 1.0,
        model_matrix: cgmath::SquareMatrix::identity(),
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png")
    }
}

use gl::types::*;

static VERTEX_DATA_2D: [GLfloat; 12] = [0.25, 1.0, 0.0, 1.0, 1.0, 0.0, 0.25, 0.25, 0.0, 1.0, 0.25, 0.0];
static UV_DATA_2D: [GLfloat; 8] = [0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
static NORMAL_DATA_2D: [GLfloat; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
static INDICES: [i16; 6] = [0,1,2,2,1,3];

pub fn io_2d() -> Model {

    Model {
        mesh: mesh::new_static_3d_mesh(VERTEX_DATA_2D.to_vec(),NORMAL_DATA_2D.to_vec(),UV_DATA_2D.to_vec(),INDICES.to_vec()),
        translation: Vector3 {
            x: -5.0,
            y: 1.0,
            z: 0.0,
        },
        rotation: cgmath::SquareMatrix::identity(),
        scale: 1.0,
        model_matrix: cgmath::SquareMatrix::identity(),
        motion: motion::new_motion(),
        texture: crate::game::loader::load_texture("./src/game/model/res/Untitled.001.png")
    }
}