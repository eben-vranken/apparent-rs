enum EquatorialError {
    DeclinationRangeError,
    NonFiniteError,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Equatorial {
    ra_rad: f64,
    dec_rad: f64,
}
