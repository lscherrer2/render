use super::super::Vec3;
use super::Quat;

impl Quat {
    pub fn rotate(self, v: Vec3) -> Vec3 {
        (self * Quat::from(v) * self.inverse()).into()
    }
}

// Vec3 * Quat Multiplication
impl std::ops::Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        (self * Quat::from(rhs)).into()
    }
}
// Vec3 * Quat Multiplication
impl std::ops::Mul<Quat> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Quat) -> Self::Output {
        (Quat::from(self) * rhs).into()
    }
}

// Vec3 <--> Quat Conversion
impl From<Quat> for Vec3 {
    fn from(value: Quat) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}
impl From<Vec3> for Quat {
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z, 0.0)
    }
}
