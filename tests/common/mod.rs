#![allow(dead_code)]

use apparent_rs::vec3::Vec3;

#[track_caller]
pub fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tolerance,
        "expected {expected}, got {actual} (diff {diff}, tolerance {tolerance})"
    );
}

#[track_caller]
pub fn assert_close_vector(actual: Vec3, expected: Vec3, tolerance: f64) {
    let diff_x = (actual.x - expected.x).abs();
    let diff_y = (actual.y - expected.y).abs();
    let diff_z = (actual.z - expected.z).abs();
    assert!(
        diff_x <= tolerance && diff_y <= tolerance && diff_z <= tolerance,
        "expected {expected:?}, got {actual:?} (diff x:{diff_x} y:{diff_y} z:{diff_z}, tolerance {tolerance})",
    );
}
