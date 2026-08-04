mod common;

use std::assert_eq;

use apparent_rs::constants::PI;
use apparent_rs::vec3::Vec3;
use common::{assert_close, assert_close_vector};

pub const TOLERANCE: f64 = 1e-12;

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
fn test_length_of_squared_skips_the_root() {
    let len = Vec3::new(3.0, 4.0, 0.0).length_squared();

    assert_eq!(len, 25.0);
}

#[test]
fn test_normalize_scales_to_unit_length() {
    let vec = Vec3::new(3.0, 4.0, 0.0).normalize();

    assert_close_vector(vec, Vec3::new(0.6, 0.8, 0.0), TOLERANCE);
}

#[test]
fn test_normalize_of_unit_vector_is_unchanged() {
    let vec = Vec3::Z.normalize();

    assert_eq!(vec, Vec3::Z);
}

#[test]
fn test_normalize_shrinks_long_vector() {
    let vec = Vec3::new(2.0, 0.0, 0.0).normalize();

    assert_eq!(vec, Vec3::X);
}

#[test]
fn test_normalize_general_vector_has_unit_length() {
    let vec = Vec3::new(1.0, 2.0, 3.0).normalize().length();

    assert_close(vec, 1.0, TOLERANCE);
}

#[test]
fn test_add() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(4.0, 5.0, 6.0);
    let sum_vec = vec_a + vec_b;

    assert_eq!(sum_vec, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_sub() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(5.0, 7.0, 9.0);

    let sub_vec = vec_b - vec_a;

    assert_eq!(sub_vec, Vec3::new(4.0, 5.0, 6.0));
}

#[test]
fn test_mul_by_scalar() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let scaled_vec = vec_a * 2.0;

    assert_eq!(scaled_vec, Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn test_mul_by_negative_scalar_reverses() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let scaled_vec = vec_a * -1.0;

    assert_eq!(scaled_vec, Vec3::new(-1.0, -2.0, -3.0));
}

#[test]
fn test_neg() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(-vec_a, Vec3::new(-1.0, -2.0, -3.0));
}

#[test]
fn test_dot_of_perpendicular_is_zero() {
    let dot = Vec3::X.dot(Vec3::Y);

    assert_eq!(dot, 0.0);
}

#[test]
fn test_dot_of_parallel_unit_vectors_is_one() {
    let dot = Vec3::X.dot(Vec3::X);

    assert_eq!(dot, 1.0);
}

#[test]
fn test_dot_of_opposite_unit_vectors_is_minus_one() {
    let dot = Vec3::X.dot(-Vec3::X);

    assert_eq!(dot, -1.0);
}

#[test]
fn test_dot_general_vectors() {
    let vec_a = Vec3::new(1.0, 2.0, 3.0);
    let vec_b = Vec3::new(4.0, 5.0, 6.0);
    let dot = vec_a.dot(vec_b);

    assert_eq!(dot, 32.0);
}

#[test]
fn test_dot_is_cosine_of_angle() {
    let vec = Vec3::new(1.0, 1.0, 0.0);
    let dot = vec.normalize().dot(Vec3::X);

    assert_close(dot, std::f64::consts::FRAC_1_SQRT_2, TOLERANCE);
}

#[test]
fn test_from_spherical_origin_is_x() {
    let vec = Vec3::from_spherical(0.0, 0.0);

    assert_eq!(vec, Vec3::X);
}

#[test]
fn test_from_spherical_quarter_turn_is_y() {
    let vec = Vec3::from_spherical(PI / 2.0, 0.0);

    assert_close_vector(vec, Vec3::Y, TOLERANCE);
}

#[test]
fn test_from_spherical_half_turn_is_negative() {
    let vec = Vec3::from_spherical(PI, 0.0);

    assert_close_vector(vec, Vec3::new(-1.0, 0.0, 0.0), TOLERANCE);
}

#[test]
fn test_from_spherical_north_pole_is_z() {
    let vec = Vec3::from_spherical(0.0, PI / 2.0);

    assert_close_vector(vec, Vec3::Z, TOLERANCE);
}

#[test]
fn test_from_spherical_south_pole_is_negative_z() {
    let vec = Vec3::from_spherical(0.0, -PI / 2.0);

    assert_close_vector(vec, -Vec3::Z, TOLERANCE);
}

#[test]
fn test_from_spherical_negative_latitude() {
    let vec = Vec3::from_spherical(0.0, -PI / 4.0);

    assert_close_vector(
        vec,
        Vec3::new(
            std::f64::consts::FRAC_1_SQRT_2,
            0.0,
            -std::f64::consts::FRAC_1_SQRT_2,
        ),
        TOLERANCE,
    );
}

#[test]
fn test_from_spherical_vega() {
    let vec = Vec3::from_spherical(279.2348, 38.78369);

    assert_close_vector(
        vec,
        Vec3::new(0.125028368297772, -0.769430155208644, 0.626372990666014),
        1e9,
    );
}

#[test]
fn test_from_spherical_is_always_unit_length() {
    let len = Vec3::from_spherical(0.0, 0.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(45.0, 30.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(135.0, -20.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(200.0, 60.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(300.0, -75.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(0.0, 89.9999).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(-30.0, 10.0).length();
    assert_close(len, 1.0, TOLERANCE);

    let len = Vec3::from_spherical(400.0, 5.0).length();
    assert_close(len, 1.0, TOLERANCE);
}

#[test]
fn test_to_spherical_x_is_origin() {
    let (lon, lat) = Vec3::X.to_spherical();

    assert_eq!(lon, 0.0);
    assert_eq!(lat, 0.0);
}

#[test]
fn test_to_spherical_y_is_quarter_turn() {
    let (lon, lat) = Vec3::Y.to_spherical();

    assert_eq!(lon, PI / 2.0);
    assert_eq!(lat, 0.0);
}

#[test]
fn test_to_spherical_z_is_pole() {
    let (lon, lat) = Vec3::Z.to_spherical();

    assert_eq!(lon, 0.0);
    assert_eq!(lat, PI / 2.0);
}

#[test]
fn test_to_spherical_negative_y_wraps_to_three_quarters() {
    let (lon, lat) = Vec3::new(0.0, -1.0, 0.0).to_spherical();

    assert_eq!(lon, 3.0 * PI / 2.0);
    assert_eq!(lat, 0.0);
}

#[test]
fn test_to_spherical_ignores_magnitude() {
    let (lon, lat) = Vec3::new(0.0, 0.0, 5.0).to_spherical();

    assert_eq!(lon, 0.0);
    assert_eq!(lat, PI / 2.0);
}
