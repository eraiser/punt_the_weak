use cgmath::Vector3;

pub struct LightSource {
    pub translation: Vector3<f32>,
    pub color: Vector3<f32>,
    pub power: f32,
}

pub fn new_light_source(translation: Vector3<f32>, color: Vector3<f32>, power: f32) -> LightSource {
    LightSource{
        translation,
        color,
        power
    }
}