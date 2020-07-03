use std::f32::consts::PI;
use cgmath::{Quaternion, Matrix4, SquareMatrix};
use std::borrow::{Borrow, BorrowMut};

pub mod motion;
pub mod transform;

pub struct Model{
    pub transform: transform::Transformation,
    pub motion: Option<motion::Motion>,
    pub current_model_matrix: Matrix4<f32>
}

pub fn new_model() -> Model{
    Model{
        transform: transform::new_neutral_model_transform(),
        motion: None,
        current_model_matrix: Matrix4::identity()
    }
}

impl Model{

    pub fn calc_model_matrix(&mut self,i_v:f32){

        self.current_model_matrix = match self.motion.borrow(){
            Some(m) => {
                let t = Matrix4::from_translation(self.transform.translation + m.movement_vector*i_v);

                let q1 = self.transform.rotation;
                let q2 = m.target_quaternion;
                let r = Matrix4::from(q1.nlerp(q2,i_v));

                let s = Matrix4::from_scale(self.transform.scale);

                t*r*s
            },
            None => {
                let t = Matrix4::from_translation(self.transform.translation);
                let r = Matrix4::from(self.transform.rotation);
                let s = Matrix4::from_scale(self.transform.scale);

                t*r*s
            }
        }
    }

    pub fn get_current_model_matrix(&self)-> Matrix4<f32>{self.current_model_matrix}

    pub fn update(&mut self){
        match self.motion.borrow_mut(){
            Some(m) => {
                self.transform.translation+= m.movement_vector;
                self.transform.rotation= m.target_quaternion;
                m.target_quaternion = self.transform.rotation * m.rotation_per_sec;
            }
            None => ()
        }
    }
}