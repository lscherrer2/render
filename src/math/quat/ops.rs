use super::super::Vec3;
use super::Quat;

impl Quat {
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }
    pub fn inverse(self) -> Self {
        self.conjugate() / Quat::dot(self, self)
    }
    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }
    pub fn norm(self) -> f32 {
        f32::sqrt(Quat::dot(self, self))
    }
    pub fn normalize(self) -> Self {
        let fac = 1.0 / self.norm();
        Self::new(self.x * fac, self.y * fac, self.z * fac, self.w * fac)
    }
    pub fn normalize_inplace(&mut self) -> &mut Self {
        let fac = 1.0 / self.norm();
        self.x *= fac;
        self.y *= fac;
        self.z *= fac;
        self.w *= fac;
        self
    }
}
impl std::ops::Mul for Quat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}
impl std::ops::Div for Quat {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}
impl std::ops::Mul<f32> for Quat {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}
impl std::ops::Div<f32> for Quat {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}
impl From<Quat> for [f32; 4] {
    fn from(value: Quat) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl From<[f32; 4]> for Quat {
    fn from(value: [f32; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}
