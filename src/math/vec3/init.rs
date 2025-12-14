use crate::math::vec3::Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{x, y, z}
    }
    pub fn zero() -> Vec3 {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }
}