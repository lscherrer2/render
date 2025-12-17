use super::Quat;

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}
impl Default for Quat {
    fn default() -> Self {
        Self::identity()
    }
}
