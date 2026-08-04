use apparent_rs::mat3::Mat3;
use apparent_rs::vec3::Vec3;

// Fixtures
const A: Mat3 = Mat3::new(
    Vec3::new(1.0, 2.0, 3.0),
    Vec3::new(4.0, 5.0, 6.0),
    Vec3::new(7.0, 8.0, 9.0),
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
