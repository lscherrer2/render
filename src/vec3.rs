pub use crate::config::Dtype;

#[derive(Clone, Copy)]
pub struct Vec3 { 
    pub x: Dtype, 
    pub y: Dtype, 
    pub z: Dtype,
}

impl Vec3 {
    pub fn new(x: Dtype, y: Dtype, z: Dtype) -> Vec3 {
        Vec3{x, y, z}
    }
    pub fn zero() -> Vec3 {
        Vec3{x: 0 as Dtype, y: 0 as Dtype, z: 0 as Dtype}
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
impl std::ops::Add<Dtype> for Vec3 {
    type Output = Self;
    fn add(self, other: Dtype) -> Self {
        Self{
            x: self.x + other, 
            y: self.y + other, 
            z: self.z + other,
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
impl std::ops::Sub<Dtype> for Vec3 {
    type Output = Self;
    fn sub(self, other: Dtype) -> Self {
        Self{
            x: self.x - other, 
            y: self.y - other, 
            z: self.z - other,
        }
    }
}
impl std::ops::Mul<Dtype> for Vec3 {
    type Output = Self;
    fn mul(self, other: Dtype) -> Self {
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl std::ops::Div<Dtype> for Vec3 {
    type Output = Self;
    fn div(self, other: Dtype) -> Self {
        Self{
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
impl Vec3 {
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
    pub fn dot(a: Self, b: Self) -> Dtype {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn norm(self) -> Dtype {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn distance(a: Self, b: Self) -> Dtype {
        (a - b).norm()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec3() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) + Vec3::new(3 as Dtype, 2 as Dtype, 1 as Dtype);
        let ans = Vec3::new(4 as Dtype, 4 as Dtype, 4 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }

    #[test]
    fn test_sub_vec3() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) - Vec3::new(3 as Dtype, 2 as Dtype, 1 as Dtype);
        let ans = Vec3::new(-2 as Dtype, 0 as Dtype, 2 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }

    #[test]
    fn test_add() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) + 1 as Dtype;
        let ans = Vec3::new(2 as Dtype, 3 as Dtype, 4 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }

    #[test]
    fn test_sub() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) - 1 as Dtype;
        let ans = Vec3::new(0 as Dtype, 1 as Dtype, 2 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }

    #[test]
    fn test_mul() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) * 2 as Dtype;
        let ans = Vec3::new(2 as Dtype, 4 as Dtype, 6 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }

    #[test]
    fn test_div() {
        let sum = Vec3::new(1 as Dtype, 2 as Dtype, 3 as Dtype) / 0.5 as Dtype;
        let ans = Vec3::new(2 as Dtype, 4 as Dtype, 6 as Dtype);
        assert!(Vec3::distance(sum, ans) < 0.0001 as Dtype);
    }
}
