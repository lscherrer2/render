use crate::math::vec3::Vec3;

#[test]
fn test_add_vec3() {
    let sum = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(3.0, 2.0, 1.0);
    let ans = Vec3::new(4.0, 4.0, 4.0);
    assert!(Vec3::distance(sum, ans) < 0.0001);
}

#[test]
fn test_sub_vec3() {
    let sum = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(3.0, 2.0, 1.0);
    let ans = Vec3::new(-2.0, 0.0, 2.0);
    assert!(Vec3::distance(sum, ans) < 0.0001);
}

#[test]
fn test_mul_scalar() {
    let sum = Vec3::new(1.0, 2.0, 3.0) * 2.0;
    let ans = Vec3::new(2.0, 4.0, 6.0);
    assert!(Vec3::distance(sum, ans) < 0.0001);
}

#[test]
fn test_div_scalar() {
    let sum = Vec3::new(1.0, 2.0, 3.0) / 0.5;
    let ans = Vec3::new(2.0, 4.0, 6.0);
    assert!(Vec3::distance(sum, ans) < 0.0001);
}

#[test]
fn test_into_arr3() {
    let a: [f32; 3] = Vec3::new(1.0, 2.0, 3.0).into();
    assert!(a[0] - 1.0 < 0.0001);
    assert!(a[1] - 2.0 < 0.0001);
    assert!(a[2] - 3.0 < 0.0001);
}

#[test]
fn test_into_arr4() {
    let a: [f32; 4] = Vec3::new(1.0, 2.0, 3.0).into();
    assert!(a[0] - 1.0 < 0.0001);
    assert!(a[1] - 2.0 < 0.0001);
    assert!(a[2] - 3.0 < 0.0001);
    assert!(a[3] - 1.0 < 0.0001);
}