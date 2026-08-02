use apparent_rs::constants;

#[test]
fn test_radians_per_degree() {
    let degrees_per_turn: f64 = constants::TAU / constants::RADIANS_PER_DEGREE;

    assert_eq!(degrees_per_turn, 360.0);
}

#[test]
fn test_radians_per_hour() {
    let hours_per_turn: f64 = constants::TAU / constants::RADIANS_PER_HOUR;

    assert_eq!(hours_per_turn, 24.0);
}

#[test]
fn test_radians_per_arcsecond() {
    let arcseconds_per_turn: f64 = constants::TAU / constants::RADIANS_PER_ARCSECOND;

    assert_eq!(arcseconds_per_turn, 1296000.0);
}

#[test]
fn test_RADIANS_PER_MILLIARCSECOND() {
    let milliarcseconds_per_turn: f64 = constants::TAU / constants::RADIANS_PER_MILLIARCSECOND;

    assert_eq!(milliarcseconds_per_turn, 1296000000.0);
}
