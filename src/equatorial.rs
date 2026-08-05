use core::fmt;

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

fn split_sexagesimal(total: f64, decimals: u32) -> (u32, u32, f64) {
    let scale = 10f64.powi(decimals as i32);
    let total = (total * scale).round() / scale;

    let first = (total / 3600.0).floor();
    let rest = total - first * 3600.0;

    let second = (rest / 60.0).floor();
    let third = rest - second * 60.0;

    (first as u32, second as u32, third)
}

impl fmt::Display for Equatorial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ra_seconds = self.ra_hours() * 3600.0;
        let (h_unnormalized, m, s) = split_sexagesimal(ra_seconds, 5);
        let h = h_unnormalized % 24;

        let sign = if self.dec_rad() < 0.0 { '-' } else { '+' };

        let dec_arcseconds = self.dec_deg().abs() * 3600.0;
        let (d, am, arcsec) = split_sexagesimal(dec_arcseconds, 4);

        write!(
            f,
            "{:02}h {:02}m {:08.5}s {}{:02}° {:02}' {:07.4}\"",
            h, m, s, sign, d, am, arcsec
        )
    }
}
