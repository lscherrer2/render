use super::Vec3;
use super::super::Quat;


impl Vec3 {
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn distance(a: Self, b: Self) -> f32 {
        (a - b).norm()
    }
}
impl std::ops::Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z,
        }
    }
}
impl std::ops::Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self{
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z,
        }
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self{
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl From<Vec3> for [f32; 3] {
    fn from(value: Vec3) -> Self {
        [value.x, value.y, value.z] 
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self::new(value[0], value[1], value[2])      
    }
}
