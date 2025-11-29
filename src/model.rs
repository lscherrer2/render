use crate::{vec3::Vec3, shape::Triangle3};

struct Model {
    faces: Vec<Triangle3>,
    origin: Vec3,
    right: Vec3,
    up: Vec3,
    forward: Vec3,
}
