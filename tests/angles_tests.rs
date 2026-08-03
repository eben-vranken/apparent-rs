mod common;

use apparent_rs::angles::{normalize_2pi, normalize_pi};
use apparent_rs::constants::{PI, TAU};
use common::assert_close;

pub const TOLERANCE: f64 = 1e-12;

#[test]
fn test_2pi_zero_is_unchanged() {
    let normalized = normalize_2pi(0.0);

    assert_eq!(normalized, 0.0);
}

#[test]
fn test_2pi_full_turn_wraps_to_zero() {
    let normalized = normalize_2pi(TAU);

    assert_eq!(normalized, 0.0);
}

#[test]
fn test_2pi_wraps_value_past_full_turn() {
    let normalized = normalize_2pi(TAU + 0.5);

    assert_close(normalized, 0.5, TOLERANCE);
}

#[test]
fn test_2pi_wraps_negative_to_positive() {
    let normalized = normalize_2pi(-0.5);

    assert_close(normalized, TAU - 0.5, TOLERANCE);
}

#[test]
fn test_2pi_wraps_negative_past_full_turn() {
    let normalized = normalize_2pi(-TAU - 0.5);

    assert_close(normalized, TAU - 0.5, TOLERANCE);
}

#[test]
fn test_2pi_tiny_negative_does_not_return_tau() {
    let normalized = normalize_2pi(-1e-20);

    assert_eq!(normalized, 0.0);
}

#[test]
fn test_2pi_precision_after_many_turns() {
    let normalized = normalize_2pi(100.0 * TAU + 1.0);

    assert_close(normalized, 1.0, TOLERANCE);
}

#[test]
fn test_pi_zero_is_unchanged() {
    let normalized = normalize_pi(0.0);

    assert_eq!(normalized, 0.0);
}

#[test]
fn test_pi_upper_boundary_maps_to_lower() {
    let normalized = normalize_pi(PI);

    assert_close(normalized, -PI, TOLERANCE);
}

#[test]
fn test_pi_lower_boundary_is_unchanged() {
    let normalized = normalize_pi(-PI);

    assert_close(normalized, -PI, TOLERANCE);
}

#[test]
fn test_pi_wraps_three_quarter_turn() {
    let normalized = normalize_pi(1.5 * PI);

    assert_close(normalized, -PI / 2.0, TOLERANCE);
}

#[test]
fn test_pi_wraps_negative_three_quarter_turn() {
    let normalized = normalize_pi(-1.5 * PI);

    assert_close(normalized, PI / 2.0, TOLERANCE);
}
