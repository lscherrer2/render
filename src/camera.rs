use crate::vec3::{Dtype, Vec3};

pub enum ProjectionMethod {
    Orthographic,
    Perspective(Dtype),
}
pub struct Camera {
    origin: Vec3,
    right: Vec3,
    up: Vec3,
}
impl Camera {
    pub fn new(origin: Vec3, right: Vec3, up: Vec3) -> Self {
        Self {origin, right, up}
    }
}
impl Camera {
    pub fn project(&self, v: Vec3, p: ProjectionMethod) -> (Dtype, Dtype) {
        // Vec3 relative to camera center
        let r = v - self.origin; 

        // Perspective factor
        let fac = match p {
            ProjectionMethod::Orthographic => 1.0 as Dtype,
            ProjectionMethod::Perspective(fd) => 
                 fd / Vec3::dot(r, Vec3::cross(self.up, self.right)),
        };

        // Project
        (Vec3::dot(r, self.right) * fac, Vec3::dot(r, self.up) * fac)
    } 
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_orthographic_project() {
        let cam = Camera::new(
            Vec3::new(1 as Dtype, 1 as Dtype, 1 as Dtype),
            Vec3::new(1 as Dtype, 0 as Dtype, 0 as Dtype),
            Vec3::new(0 as Dtype, 0 as Dtype, 1 as Dtype),
        );
        let vec = Vec3::new(2 as Dtype, 1 as Dtype, 2 as Dtype);
        let prj = cam.project(vec, ProjectionMethod::Orthographic);
        let ans = (1 as Dtype, 1 as Dtype);
        assert!(
            Vec3::distance(
                Vec3::new(prj.0, prj.1, 0 as Dtype),
                Vec3::new(ans.0, ans.1, 0 as Dtype),
            ) < 0.000001
        );
    }

    #[test]
    pub fn test_perspective_project() {
        let cam = Camera::new(
            Vec3::new(1 as Dtype, 1 as Dtype, 1 as Dtype),
            Vec3::new(1 as Dtype, 0 as Dtype, 0 as Dtype),
            Vec3::new(0 as Dtype, 0 as Dtype, 1 as Dtype),
        );
        let fd = 5.0 as Dtype;
        let vec = Vec3::new(2 as Dtype, 1 as Dtype, 2 as Dtype);
        let prj = cam.project(vec, ProjectionMethod::Perspective(fd));
        // let ans = ... // TODO: generate actual test case
    }
}

