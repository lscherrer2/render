use super::Object;
use super::Transform;
use crate::math::Vec3;

pub struct Camera {
    tf: Transform,
}
impl Object for Camera {
    fn get_transform(&self) -> Transform {
        self.tf
    }
    fn set_transform(&mut self, tf: Transform) {
        self.tf = tf;
    }
    fn get_transform_ref_mut(&mut self) -> &mut Transform {
        &mut self.tf
    }
}
impl Camera {
    fn project(self, v: Vec3) -> [f32; 2] {
        let r = self.tf.right();
        let u = self.tf.up();
        let x = Vec3::dot(v, r) / Vec3::dot(r, r);
        let y = Vec3::dot(v, u) / Vec3::dot(u, u);
        [x, y]
    }
    fn project_triangle(self, t: [Vec3; 3]) -> [[f32; 2]; 3] {
        let r = self.tf.right();
        let u = self.tf.up();
        let rd = Vec3::dot(r, r);
        let ud = Vec3::dot(u, u);
        [
            [Vec3::dot(t[0], r) / rd, Vec3::dot(t[0], u) / ud],
            [Vec3::dot(t[1], r) / rd, Vec3::dot(t[1], u) / ud],
            [Vec3::dot(t[2], r) / rd, Vec3::dot(t[2], u) / ud],
        ]
    }
    fn project_traingles(self, t: &Vec<[Vec3; 3]>) -> Vec<[[f32; 2]; 3]> {
        let r = self.tf.right();
        let u = self.tf.up();
        let rd = Vec3::dot(r, r);
        let ud = Vec3::dot(u, u);
        t.iter()
            .map(|t| {
                [
                    [Vec3::dot(t[0], r) / rd, Vec3::dot(t[0], u) / ud],
                    [Vec3::dot(t[1], r) / rd, Vec3::dot(t[1], u) / ud],
                    [Vec3::dot(t[2], r) / rd, Vec3::dot(t[2], u) / ud],
                ]
            })
            .collect()
    }
}
