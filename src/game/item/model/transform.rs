

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3, Quaternion, Rotation3, One};

pub struct Transformation {
    pub rotation: Quaternion<f32>,
    pub translation: Vector3<f32>,
    pub scale: f32,
}

pub fn new_neutral_model_transform() -> Transformation {

    Transformation {
        rotation: Quaternion::one(),
        translation: Vector3::new(0.0,0.0,0.0),
        scale: 1.0,
    }
}

impl Transformation {

    pub fn rotate_x(&mut self, angle: Rad<f32>){
        self.rotation = self.rotation * Quaternion::from_axis_angle(Vector3::unit_x(),angle);
    }
    pub fn rotate_y(&mut self, angle: Rad<f32>){
        self.rotation = self.rotation * Quaternion::from_axis_angle(Vector3::unit_y(),angle);
    }
    pub fn rotate_z(&mut self, angle: Rad<f32>){
        self.rotation = self.rotation * Quaternion::from_axis_angle(Vector3::unit_z(),angle);
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale *= scale;
    }
    pub fn translate(&mut self, translate: Vector3<f32>) {
        self.translation += translate;
    }

}
