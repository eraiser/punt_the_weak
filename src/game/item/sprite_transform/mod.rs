use std::f32::consts::PI;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector2};

pub struct SpriteTransform {
    rotation_r: Rad<f32>,
    rotation_matrix: Option<Matrix4<f32>>,
    offset: Vector2<f32>,
    scale: f32,
}

pub fn new_sprite_transform() -> SpriteTransform
{
    SpriteTransform{
        rotation_r: Rad(0.0),
        rotation_matrix: None,
        offset: Vector2{ x: 0.0, y: 0.0 },
        scale: 1.0
    }
}
impl SpriteTransform{
    pub fn get_offset(&self) -> Vector2<f32> {
        self.offset
    }
    pub fn set_offset(&mut self, v: Vector2<f32>) {
        self.offset = v;
    }
    pub fn set_scale(&mut self, s: f32) {
        self.scale = s;
    }
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}