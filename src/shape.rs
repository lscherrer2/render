use crate::vec3::{Vec3, Dtype};

pub type Point = (Dtype, Dtype);

pub struct Triangle3(pub Vec3, pub Vec3, pub Vec3);
pub struct Triangle2(pub Point, pub Point, pub Point);


