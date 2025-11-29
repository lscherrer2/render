use crate::{mat::Mat4, vec3::Vec3, config::Dtype};
pub enum Frame {Local, Global}

pub struct Transformation { mat: Mat4 }

/// Constructors
impl Transformation {
    fn new(x: Vec3, y: Vec3, z: Vec3, origin: Vec3) -> Self {
        Self{mat: Mat4{data: [
            x.x, y.x, z.x, origin.x,
            x.y, y.y, z.y, origin.y,
            x.z, y.z, z.z, origin.z,
            0 as Dtype, 0 as Dtype, 0 as Dtype, 1 as Dtype,
        ]}}
    }
    fn default() -> Self {
        Self{mat: Mat4{data: [
            1 as Dtype, 0 as Dtype, 0 as Dtype, 0 as Dtype,
            0 as Dtype, 1 as Dtype, 0 as Dtype, 0 as Dtype,
            0 as Dtype, 0 as Dtype, 1 as Dtype, 0 as Dtype,
            0 as Dtype, 0 as Dtype, 0 as Dtype, 1 as Dtype,
        ]}}
    }
}

