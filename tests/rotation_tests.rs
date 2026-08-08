mod common;

use apparent_rs::direction::Direction;
use apparent_rs::frames::{Icrs, MeanOfDate};
use apparent_rs::mat3::Mat3;
use apparent_rs::rotation::Rotation;
use common::assert_close_vector;

const TOLERANCE: f64 = 1e-15;

fn sample_direction() -> Direction<Icrs> {
    Direction::from_spherical(0.9, 0.4)
}

#[test]
fn test_identity_leaves_direction_unchanged() {
    let r: Rotation<Icrs, Icrs> = Rotation::new(Mat3::IDENTITY);
    let d = sample_direction();

    assert_close_vector((r * d).as_vec3(), d.as_vec3(), TOLERANCE);
}

#[test]
fn test_inverse_round_trips() {
    let r: Rotation<Icrs, MeanOfDate> = Rotation::new(Mat3::rz(0.3));
    let d = sample_direction();

    assert_close_vector((r.inverse() * (r * d)).as_vec3(), d.as_vec3(), TOLERANCE);
}

#[test]
fn test_composition_is_associative() {
    let r1: Rotation<Icrs, MeanOfDate> = Rotation::new(Mat3::rz(0.3));
    let r2: Rotation<MeanOfDate, Icrs> = Rotation::new(Mat3::rx(0.7));
    let d = sample_direction();

    assert_close_vector(
        ((r2 * r1) * d).as_vec3(),
        (r2 * (r1 * d)).as_vec3(),
        TOLERANCE,
    );
}

#[test]
fn test_then_matches_multiplication_order() {
    let r1: Rotation<Icrs, MeanOfDate> = Rotation::new(Mat3::rz(0.3));
    let r2: Rotation<MeanOfDate, Icrs> = Rotation::new(Mat3::rx(0.7));

    assert_eq!(r1.then(r2), r2 * r1);
}
