mod mesh;
pub mod motion;
use cgmath::{Matrix4, Vector4, Rad, Vector3};

pub struct Model {
    mesh: mesh::Mesh,
    rotation: Matrix4<f32>,
    translation: Vector3<f32>,
    scale: f32,
    model_matrix: Matrix4<f32>,
    motion: motion::Motion
}

impl Model {
    pub fn draw_2d(&self) {
        self.mesh.draw_2d();
    }
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
    pub fn scale(&mut self, scale: f32) { self.scale *= scale; }
    pub fn translate(&mut self, translate: Vector3<f32>) {
        self.translation += translate;
    }

    pub fn update(&mut self) {
        self.model_matrix = Matrix4::from_translation(self.translation) * self.rotation * self.scale;
    }
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        self.model_matrix
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"(modelmatrix {:?}",self.model_matrix)
    }
}

use gl::types::*;


// Vertex data
static VERTEX_DATA_2D: [GLfloat; 6] = [0.0, 0.5,
    0.5, -0.5,
    -0.5, -0.5];

static VERTEX_DATA_3D: [GLfloat; 9] = [0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0];

pub fn triangle() -> Model {
    Model {
        mesh: mesh::new_static_3d_mesh(VERTEX_DATA_3D.to_vec()),
        translation: Vector3{x: 0.0,y: 0.0,z: 0.0},
        rotation: cgmath::SquareMatrix::identity(),
        scale: 1.0,
        model_matrix: cgmath::SquareMatrix::identity(),
        motion: motion::new_motion()
    }
}
