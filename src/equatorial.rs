use crate::{
    angles::normalize_2pi,
    constants::{PI, RADIANS_PER_DEGREE},
};

#[derive(Debug, PartialEq)]
pub enum EquatorialError {
    DeclinationRangeError,
    NonFiniteError,
}

// Private fields, so I can validate them.
// Declination: -90 to +90 (degrees)
// Right ascension: 0 - 360 (degrees)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Equatorial {
    ra_rad: f64,
    dec_rad: f64,
}

impl Equatorial {
    pub fn new(ra_rad: f64, dec_rad: f64) -> Result<Self, EquatorialError> {
        if !((-PI / 2.0)..=(PI / 2.0)).contains(&dec_rad) {
            return Err(EquatorialError::DeclinationRangeError);
        }

        let right_asc = normalize_2pi(ra_rad);

        Ok(Self {
            ra_rad: right_asc,
            dec_rad,
        })
    }

    pub fn ra_rad(&self) -> f64 {
        self.ra_rad
    }

    pub fn dec_rad(&self) -> f64 {
        self.dec_rad
    }

    pub fn ra_deg(&self) -> f64 {
        return self.ra_rad / RADIANS_PER_DEGREE;
    }

    pub fn dec_deg(&self) -> f64 {
        return self.dec_rad / RADIANS_PER_DEGREE;
    }

    pub fn ra_hours(&self) -> f64 {
        return self.ra_deg() / 15.0;
    }
}
