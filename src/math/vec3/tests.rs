use crate::math::vec3::Vec3;

fn assert_f32_near(actual: f32, expected: f32, eps: f32) {
    assert!(
        (actual - expected).abs() <= eps,
        "expected {expected}, got {actual} (eps={eps})"
    );
}

fn assert_vec3_near(actual: Vec3, expected: Vec3, eps: f32) {
    assert!(Vec3::distance(actual, expected) <= eps, "vec mismatch");
}

#[test]
fn test_new_and_zero() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_f32_near(v.x, 1.0, 1e-6);
    assert_f32_near(v.y, 2.0, 1e-6);
    assert_f32_near(v.z, 3.0, 1e-6);

    let z = Vec3::zero();
    assert_vec3_near(z, Vec3::new(0.0, 0.0, 0.0), 1e-6);
}

#[test]
fn test_dot_cross_norm_distance() {
    let ex = Vec3::new(1.0, 0.0, 0.0);
    let ey = Vec3::new(0.0, 1.0, 0.0);
    let ez = Vec3::new(0.0, 0.0, 1.0);

    assert_f32_near(Vec3::dot(ex, ex), 1.0, 1e-6);
    assert_f32_near(Vec3::dot(ex, ey), 0.0, 1e-6);
    assert_f32_near(Vec3::dot(ey, ex), 0.0, 1e-6);

    // Right-hand rule: ex x ey = ez
    assert_vec3_near(Vec3::cross(ex, ey), ez, 1e-6);
    assert_vec3_near(Vec3::cross(ey, ex), Vec3::new(0.0, 0.0, -1.0), 1e-6);

    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_f32_near(v.norm(), 5.0, 1e-6);
    assert_f32_near(Vec3::distance(v, Vec3::zero()), 5.0, 1e-6);
}

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
fn test_from_arr3() {
    let v: Vec3 = [1.0, 2.0, 3.0].into();
    assert_vec3_near(v, Vec3::new(1.0, 2.0, 3.0), 1e-6);
}
