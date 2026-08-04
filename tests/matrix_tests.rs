mod common;

use apparent_rs::constants::PI;
use apparent_rs::mat3::Mat3;
use apparent_rs::vec3::Vec3;
use common::assert_close_vector;

const TOLERANCE: f64 = 1e-12;

// Fixtures
const A: Mat3 = Mat3::new(
    Vec3::new(1.0, 2.0, 3.0),
    Vec3::new(4.0, 5.0, 6.0),
    Vec3::new(7.0, 8.0, 9.0),
);

const B: Mat3 = Mat3::new(
    Vec3::new(9.0, 8.0, 7.0),
    Vec3::new(6.0, 5.0, 4.0),
    Vec3::new(3.0, 2.0, 1.0),
);

#[test]
fn test_identity_times_vector_is_unchanged() {
    let vec = Mat3::IDENTITY * Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(vec, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_multiply_by_vector_general() {
    let vec = A * Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(vec, Vec3::new(14.0, 32.0, 50.0));
}

#[test]
fn test_multiply_by_vector_uses_rows() {
    let vec = A * Vec3::X;

    assert_eq!(vec, Vec3::new(1.0, 4.0, 7.0));
}

#[test]
fn test_identity_times_matrix_is_unchanged() {
    let matrix = Mat3::IDENTITY * A;

    assert_eq!(matrix, A);
}

#[test]
fn test_matrix_times_identity_is_unchanged() {
    let matrix = A * Mat3::IDENTITY;

    assert_eq!(matrix, A);
}

#[test]
fn test_multiply_by_matrix_general() {
    let matrix = A * B;

    assert_eq!(
        matrix,
        Mat3::new(
            Vec3::new(30.0, 24.0, 18.0),
            Vec3::new(84.0, 69.0, 54.0),
            Vec3::new(138.0, 114.0, 90.0),
        )
    );
}

#[test]
fn test_multiply_matrix_is_not_commutative() {
    let matrix_a = A * B;
    let matrix_b = B * A;

    assert_ne!(
        matrix_a, matrix_b,
        "Expected non equality, but got equality"
    );
}

#[test]
fn test_multiply_by_matrix_composes() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let vec = (A * B) * v;

    assert_eq!(vec, A * (B * v));
}

#[test]
fn test_transpose_of_identity_is_identity() {
    let matrix = Mat3::IDENTITY.transpose();

    assert_eq!(matrix, Mat3::IDENTITY);
}

#[test]
fn test_transpose_swaps_rows_and_columns() {
    let matrix = A.transpose();

    assert_eq!(
        matrix,
        Mat3::new(
            Vec3::new(1.0, 4.0, 7.0),
            Vec3::new(2.0, 5.0, 8.0),
            Vec3::new(3.0, 6.0, 9.0),
        )
    );
}

#[test]
fn test_transpose_twice_is_original() {
    let matrix = A.transpose().transpose();

    assert_eq!(matrix, A);
}

#[test]
fn test_rx_of_zero_is_identity() {
    let matrix = Mat3::rx(0.0);

    assert_eq!(matrix, Mat3::IDENTITY);
}

#[test]
fn test_ry_of_zero_is_identity() {
    let matrix = Mat3::ry(0.0);

    assert_eq!(matrix, Mat3::IDENTITY);
}

#[test]
fn test_rz_of_zero_is_identity() {
    let matrix = Mat3::rz(0.0);

    assert_eq!(matrix, Mat3::IDENTITY);
}

#[test]
fn test_rx_leaves_x_axis_fixed() {
    let vec = Mat3::rx(0.7) * Vec3::X;

    assert_eq!(vec, Vec3::X);
}

#[test]
fn test_ry_leaves_y_axis_fixed() {
    let vec = Mat3::ry(0.7) * Vec3::Y;

    assert_eq!(vec, Vec3::Y);
}

#[test]
fn test_rz_leaves_z_axis_fixed() {
    let vec = Mat3::rz(0.7) * Vec3::Z;

    assert_eq!(vec, Vec3::Z);
}

#[test]
fn test_rz_quarter_turn_moves_x_to_negative_y() {
    let vec = Mat3::rz(PI / 2.0) * Vec3::X;

    assert_close_vector(vec, Vec3::new(0.0, -1.0, 0.0), TOLERANCE);
}

#[test]
fn test_rx_quarter_turn_moves_y_to_negative_z() {
    let vec = Mat3::rx(PI / 2.0) * Vec3::Y;

    assert_close_vector(vec, Vec3::new(0.0, 0.0, -1.0), TOLERANCE);
}

#[test]
fn test_ry_quarter_turn_moves_z_to_negative_x() {
    let vec = Mat3::ry(PI / 2.0) * Vec3::Z;

    assert_close_vector(vec, Vec3::new(-1.0, 0.0, 0.0), TOLERANCE);
}

#[test]
fn test_rz_angles_add() {
    let matrix = Mat3::rz(0.3) * Mat3::rz(0.4);

    assert_close_vector(matrix.row_x, Mat3::rz(0.7).row_x, TOLERANCE);
    assert_close_vector(matrix.row_y, Mat3::rz(0.7).row_y, TOLERANCE);
    assert_close_vector(matrix.row_z, Mat3::rz(0.7).row_z, TOLERANCE);
}

#[test]
fn test_rz_times_transpose_is_identity() {
    let matrix = Mat3::rz(0.7) * Mat3::rz(0.7).transpose();

    assert_close_vector(matrix.row_x, Mat3::IDENTITY.row_x, TOLERANCE);
    assert_close_vector(matrix.row_y, Mat3::IDENTITY.row_y, TOLERANCE);
    assert_close_vector(matrix.row_z, Mat3::IDENTITY.row_z, TOLERANCE);
}

#[test]
fn test_rotations_preserve_length() {
    let v = Vec3::new(1.0, 2.0, 3.0).normalize();

    let rotations = [
        ("rx", Mat3::rx(0.7)),
        ("ry", Mat3::ry(0.7)),
        ("rz", Mat3::rz(0.7)),
    ];

    for (name, matrix) in rotations {
        let len = (matrix * v).length();
        let diff = (len - 1.0).abs();

        assert!(
            diff <= TOLERANCE,
            "{name}: expected unit length, got {len} (diff {diff})"
        );
    }
}

#[test]
fn test_rotations_do_not_commute() {
    let matrix_a = Mat3::rz(0.5) * Mat3::ry(0.5);
    let matrix_b = Mat3::ry(0.5) * Mat3::rz(0.5);
    assert_ne!(matrix_a, matrix_b);
}

#[test]
fn test_rotation_determinants_are_plus_one() {
    let rotations = [
        ("rx", Mat3::rx(0.7)),
        ("ry", Mat3::ry(0.7)),
        ("rz", Mat3::rz(0.7)),
    ];

    for (name, matrix) in rotations {
        let det = matrix.determinant();
        let diff = (det - 1.0).abs();

        assert!(
            diff <= TOLERANCE,
            "{name}: expected determinant +1, got {det} (diff {diff})"
        );
    }
}

#[test]
fn test_determinant_of_identity_is_one() {
    let det = Mat3::IDENTITY.determinant();
    assert_eq!(det, 1.0);
}

#[test]
fn test_determinant_of_singular_matrix_is_zero() {
    let det = A.determinant();
    assert_eq!(det, 0.0);
}

#[test]
fn test_determinant_of_reflection_is_minus_one() {
    let det = Mat3::new(Vec3::X, Vec3::Y, Vec3::new(0.0, 0.0, -1.0)).determinant();
    assert_eq!(det, -1.0);
}
