use crate::angles::normalize_2pi;
use crate::constants::{PI, RADIANS_PER_DEGREE, RADIANS_PER_HOUR};
use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum EquatorialError {
    DeclinationRange,
    NonFinite,
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
        if !ra_rad.is_finite() || !dec_rad.is_finite() {
            return Err(EquatorialError::NonFinite);
        }

        if !((-PI / 2.0)..=(PI / 2.0)).contains(&dec_rad) {
            return Err(EquatorialError::DeclinationRange);
        }

        let right_asc = normalize_2pi(ra_rad);

        Ok(Self {
            ra_rad: right_asc,
            dec_rad,
        })
    }

    pub fn from_degrees(ra_deg: f64, dec_deg: f64) -> Result<Self, EquatorialError> {
        Equatorial::new(ra_deg * RADIANS_PER_DEGREE, dec_deg * RADIANS_PER_DEGREE)
    }

    pub fn ra_rad(&self) -> f64 {
        self.ra_rad
    }

    pub fn dec_rad(&self) -> f64 {
        self.dec_rad
    }

    pub fn ra_deg(&self) -> f64 {
        self.ra_rad / RADIANS_PER_DEGREE
    }

    pub fn dec_deg(&self) -> f64 {
        self.dec_rad / RADIANS_PER_DEGREE
    }

    pub fn ra_hours(&self) -> f64 {
        self.ra_rad / RADIANS_PER_HOUR
    }

    // Doesn't need new since to_spherical can't produce an out-of-range declination
    pub fn from_vec3(v: Vec3) -> Self {
        let (ra_rad, dec_rad) = v.to_spherical();

        Self { ra_rad, dec_rad }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::from_spherical(self.ra_rad(), self.dec_rad())
    }
}
