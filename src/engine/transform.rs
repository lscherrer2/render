use crate::math::{Matrix, Quat, Vec3};

#[derive(Clone, Copy, Default)]
pub struct Transform {
    rotation: Quat,
    translation: Vec3,
}

impl Transform {
    pub fn new(rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
        }
    }
    pub fn apply(self, v: Vec3) -> Vec3 {
        self.translation + self.rotation.rotate(v)
    }
    pub fn rotate(self, v: Vec3) -> Vec3 {
        self.rotation.rotate(v)
    }
    pub fn translate(self, v: Vec3) -> Vec3 {
        self.translation + v
    }
    pub fn to_affine(self) -> Matrix<4, 4> {
        let x = self.rotation.rotate(Vec3::new(1.0, 0.0, 0.0));
        let y = self.rotation.rotate(Vec3::new(0.0, 1.0, 0.0));
        let z = self.rotation.rotate(Vec3::new(0.0, 0.0, 1.0));
        let t = self.translation;
        Matrix::from_cols([
            [x.x, x.y, x.z, 0.0],
            [y.x, y.y, y.z, 0.0],
            [z.x, z.y, z.z, 0.0],
            [t.x, t.y, t.z, 1.0],
        ])
    }
    pub fn combine(lhs: Self, rhs: Self) -> Self {
        Self::new(lhs.rotation * rhs.rotation, lhs.apply(rhs.translation))
    }
    pub fn inverse(self) -> Self {
        Self::new(self.rotation.inverse(), -self.translation)
    }
    pub fn forward(self) -> Vec3 {
        self.rotate(Vec3::new(1.0, 0.0, 0.0))
    }
    pub fn right(self) -> Vec3 {
        self.rotate(Vec3::new(0.0, 1.0, 0.0))
    }
    pub fn up(self) -> Vec3 {
        self.rotate(Vec3::new(0.0, 0.0, 1.0))
    }
}

impl From<Transform> for Matrix<4, 4> {
    fn from(value: Transform) -> Self {
        value.to_affine()
    }
}
