use std::f32::consts::PI;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector2};

pub struct SpriteTransform {
    rotation: Rad<f32>,
    rotation_matrix: Option<Matrix4<f32>>,
    offset: Vector2<f32>,
    dimensions: Vector2<f32>,
}

pub fn new_sprite_transform() -> SpriteTransform
{
    SpriteTransform{
        rotation: Rad(0.0),
        rotation_matrix: None,
        offset: Vector2{ x: 0.0, y: 0.0 },
        dimensions: Vector2{ x: 0.0, y: 0.0 }
    }
}
impl SpriteTransform{
    pub fn get_offset(&self) -> Vector2<f32> {
        self.offset
    }
    pub fn set_offset(&mut self, v: Vector2<f32>) {
        self.offset = v;
    }
    pub fn get_dimensions(&self) -> Vector2<f32> {
        self.dimensions
    }
    pub fn set_dimensions(&mut self, d: Vector2<f32>) {
        self.dimensions = d;
    }
}