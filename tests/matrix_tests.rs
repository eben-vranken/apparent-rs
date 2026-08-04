use apparent_rs::mat3::Mat3;
use apparent_rs::vec3::Vec3;

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
    let matrix = Mat3::IDENTITY * Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(matrix, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_multiply_by_vector_general() {
    let matrix = A * Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(matrix, Vec3::new(14.0, 32.0, 50.0));
}

#[test]
fn test_multiply_by_vector_uses_rows() {
    let matrix = A * Vec3::X;

    assert_eq!(matrix, Vec3::new(1.0, 4.0, 7.0));
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
    let matrix = (A * B) * v;

    assert_eq!(matrix, A * (B * v));
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
