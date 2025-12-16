use super::Quat;
use super::super::Vec3;

fn assert_f32_near(actual: f32, expected: f32, eps: f32) {
    assert!(
        (actual - expected).abs() <= eps,
        "expected {expected}, got {actual} (eps={eps})"
    );
}

fn assert_vec3_near(actual: Vec3, expected: Vec3, eps: f32) {
    assert!((actual - expected).norm() <= eps, "vec mismatch");
}

fn assert_quat_near(actual: Quat, expected: Quat, eps: f32) {
    assert_f32_near(actual.x, expected.x, eps);
    assert_f32_near(actual.y, expected.y, eps);
    assert_f32_near(actual.z, expected.z, eps);
    assert_f32_near(actual.w, expected.w, eps);
}

#[test]
fn test_new() {
    let quat = Quat::new(1.0, 2.0, 3.0, 4.0);
    assert!(f32::abs(quat.x - 1.0) < 0.00001);
    assert!(f32::abs(quat.y - 2.0) < 0.00001);
    assert!(f32::abs(quat.z - 3.0) < 0.00001);
    assert!(f32::abs(quat.w - 4.0) < 0.00001);
}

#[test]
fn test_identity() {
    let quat = Quat::identity();
    assert!(f32::abs(quat.x - 0.0) < 0.00001);
    assert!(f32::abs(quat.y - 0.0) < 0.00001);
    assert!(f32::abs(quat.z - 0.0) < 0.00001);
    assert!(f32::abs(quat.w - 1.0) < 0.00001);
}

#[test]
fn test_dot_norm_normalize() {
    let q = Quat::new(1.0, 2.0, 3.0, 4.0);
    assert_f32_near(Quat::dot(q, q), 1.0 + 4.0 + 9.0 + 16.0, 1e-6);
    assert_f32_near(q.norm(), (30.0f32).sqrt(), 1e-6);

    let qn = q.normalize();
    assert_f32_near(qn.norm(), 1.0, 1e-5);

    let mut qm = q;
    qm.normalize_inplace();
    assert_f32_near(qm.norm(), 1.0, 1e-5);
}

#[test]
fn test_mul_identity_is_noop() {
    let q = Quat::new(0.1, -0.2, 0.3, 0.9).normalize();
    let i = Quat::identity();
    assert_quat_near(i * q, q, 1e-6);
    assert_quat_near(q * i, q, 1e-6);
}

#[test]
fn test_conjugate_and_inverse_for_unit_quat() {
    // For unit quaternions, inverse == conjugate and q*q^{-1} == identity.
    let q = Quat::new(0.2, -0.3, 0.4, 0.5).normalize();
    assert_quat_near(q.inverse(), q.conjugate(), 1e-5);

    let prod = q * q.inverse();
    assert_quat_near(prod, Quat::identity(), 1e-5);

    // Also true: q*conjugate(q) == identity for unit q
    let prod2 = q * q.conjugate();
    assert_quat_near(prod2, Quat::identity(), 1e-5);
}

#[test]
fn test_conjugate_is_involution() {
    let q = Quat::new(1.0, 2.0, 3.0, 4.0);
    assert_quat_near(q.conjugate().conjugate(), q, 1e-6);
}

#[test]
fn test_from_into_arr4() {
    let q = Quat::new(1.0, 2.0, 3.0, 4.0);
    let a: [f32; 4] = q.into();
    assert_f32_near(a[0], 1.0, 1e-6);
    assert_f32_near(a[1], 2.0, 1e-6);
    assert_f32_near(a[2], 3.0, 1e-6);
    assert_f32_near(a[3], 4.0, 1e-6);

    let q2: Quat = a.into();
    assert_quat_near(q2, Quat::new(1.0, 2.0, 3.0, 4.0), 1e-6);
}

#[test]
fn test_vec3_quat_conversions_drop_w() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let q: Quat = v.into();
    assert_quat_near(q, Quat::new(1.0, 2.0, 3.0, 0.0), 1e-6);
    let v2: Vec3 = Quat::new(1.0, 2.0, 3.0, 99.0).into();
    assert_vec3_near(v2, Vec3::new(1.0, 2.0, 3.0), 1e-6);
}

#[test]
fn test_quat_vec3_mul_identity_is_noop() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let i = Quat::identity();
    assert_vec3_near(i * v, v, 1e-6);
    assert_vec3_near(v * i, v, 1e-6);
}

#[test]
fn test_inv_is_true_inverse_for_non_unit_quat() {
    // For a general quaternion q != 0, q * q^{-1} should equal identity.
    let q = Quat::new(1.0, 2.0, 3.0, 4.0);
    let prod = q * q.inverse();
    assert_quat_near(prod, Quat::identity(), 1e-5);
}

#[test]
fn test_quat_div_matches_mul_inverse() {
    let q = Quat::new(1.0, 2.0, 3.0, 4.0);
    let r = Quat::new(-0.5, 0.25, 2.0, -1.0);

    let a = q / r;
    let b = q * r.inverse();
    assert_quat_near(a, b, 1e-5);

    let c = q / q;
    assert_quat_near(c, Quat::identity(), 1e-5);
}

#[test]
fn test_scalar_mul_div_roundtrip() {
    let q = Quat::new(1.0, -2.0, 3.0, -4.0);
    assert_quat_near((q * 2.0) / 2.0, q, 1e-6);
    assert_quat_near((q / 2.0) * 2.0, q, 1e-6);
}

#[test]
fn test_quat_vec3_mul_is_pure_quat_mul_not_rotation() {
    // Current semantics in quat/vec3.rs:
    //   q * v == (q * Quat::from(v)).into()  (drops w)
    // This is *not* the usual rotation formula q * pure(v) * q^{-1}.
    // This test locks in the current behavior.

    // Example: q = (0,0,1,0), v=(1,0,0)
    // q * (1,0,0,0) = (0,1,0,0) => Vec3(0,1,0)
    let q = Quat::new(0.0, 0.0, 1.0, 0.0);
    let v = Vec3::new(1.0, 0.0, 0.0);
    let out = q * v;
    assert_vec3_near(out, Vec3::new(0.0, 1.0, 0.0), 1e-6);
}