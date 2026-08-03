#[track_caller]
pub fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tolerance,
        "expected {expected}, got {actual} (diff {diff}, tolerance {tolerance})"
    );
}
