use amethyst::core::{
    math::{Vector2, Vector3},
    Float,
};

pub fn vector2_to_f32(v: &Vector2<Float>) -> Vector2<f32> {
    Vector2::new(v[0].as_f32(), v[1].as_f32())
}

pub fn vector3_to_f32(v: &Vector3<Float>) -> Vector3<f32> {
    Vector3::new(v[0].as_f32(), v[1].as_f32(), v[2].as_f32())
}
