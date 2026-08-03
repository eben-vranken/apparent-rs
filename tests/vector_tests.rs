mod common;

use apparent_rs::vec3::Vec3;
use common::assert_close_vector;

pub const TOLERANCE: f64 = 1e-9;

#[test]
fn test_length_of_unit_x() {
    let len = Vec3::X.length();

    assert_eq!(len, 1.0);
}

#[test]
fn test_length_three_four() {
    let len = Vec3::new(3.0, 4.0, 0.0).length();

    assert_eq!(len, 5.0);
}

#[test]
fn test_length_of_zero_vector() {
    let len = Vec3::ZERO.length();

    assert_eq!(len, 0.0);
}

#[test]
fn test_length_of_squared_skkips_the_root() {
    let len = Vec3::new(3.0, 4.0, 0.0).length_squared();

    assert_eq!(len, 25.0);
}

#[test]
fn test_normalize_scales_to_unit_length() {
    let vec = Vec3::new(3.0, 4.0, 0.0).normalize();

    assert_close_vector(vec, Vec3::new(0.6, 0.8, 0.0), TOLERANCE);
}
