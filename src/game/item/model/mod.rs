use std::f32::consts::PI;
use cgmath::{Quaternion, Matrix4, SquareMatrix, One};
use std::borrow::{Borrow, BorrowMut};
use crate::game::item::model::motion::MotionType::q_to_q;

pub mod motion;
pub mod transform;

pub struct Model{
    pub transform: transform::Transformation,
    pub motion: Option<motion::MotionType>,
    pub current_model_matrix: Matrix4<f32>
}

pub fn new_model() -> Model{
    Model{
        transform: transform::new_neutral_model_transform(),
        motion: None,
        current_model_matrix: Matrix4::identity()
    }
}

use motion::MotionType;

impl Model{

    pub fn calc_model_matrix(&mut self,i_v:f32){

        self.current_model_matrix = match self.motion.borrow(){
            Some(m) => {
                match m {
                    MotionType::continuous(cm) => {

                        let t = Matrix4::from_translation(self.transform.translation + cm.movement_vector*i_v);

                        let q1 = self.transform.rotation;
                        let q2 = cm.rotation_per_sec;
                        let r =  Matrix4::from(Quaternion::one().nlerp(q2,i_v)*q1);

                        let s = Matrix4::from_scale(self.transform.scale);

                        t*r*s
                    }
                    MotionType::q_to_q(qm) => {

                        let t = Matrix4::from_translation(self.transform.translation);

                        let q1 = self.transform.rotation;
                        let r =  Matrix4::from(qm.last_quaternion.nlerp(qm.target_quaternion,i_v) * q1);

                        let s = Matrix4::from_scale(self.transform.scale);

                        t*r*s
                    }
                }
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

        match self.motion.borrow_mut() {
            Some(m) => {
                match m {
                    MotionType::continuous(cm) => {
                        self.transform.rotation = cm.rotation_per_sec * self.transform.rotation ;
                        self.transform.translation += cm.movement_vector;
                    }
                    MotionType::q_to_q(qm) => {
                        qm.last_quaternion = qm.target_quaternion ;
                    }
                }
            },
            _ => {}
        }
    }
}