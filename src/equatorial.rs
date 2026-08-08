use core::fmt;
use std::marker::PhantomData;

use crate::angles::normalize_2pi;
use crate::constants::{PI, RADIANS_PER_DEGREE, RADIANS_PER_HOUR};
use crate::direction::Direction;
use crate::frames::Frame;

#[derive(Debug, PartialEq)]
pub enum EquatorialError {
    DeclinationRange,
    NonFinite,
}

// Private fields, so I can validate them.
// Declination: -90 to +90 (degrees)
// Right ascension: 0 - 360 (degrees)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Equatorial<F: Frame> {
    ra_rad: f64,
    dec_rad: f64,
    frame: PhantomData<F>,
}

impl<F: Frame> Equatorial<F> {
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
            frame: PhantomData,
        })
    }

    pub fn from_degrees(ra_deg: f64, dec_deg: f64) -> Result<Self, EquatorialError> {
        Self::new(ra_deg * RADIANS_PER_DEGREE, dec_deg * RADIANS_PER_DEGREE)
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

    pub fn from_direction(d: Direction<F>) -> Self {
        let (ra_rad, dec_rad) = d.to_spherical();

        Self {
            ra_rad,
            dec_rad,
            frame: PhantomData,
        }
    }

    pub fn to_direction(&self) -> Direction<F> {
        Direction::from_spherical(self.ra_rad(), self.dec_rad())
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

impl<F: Frame> fmt::Display for Equatorial<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ra_seconds = self.ra_hours() * 3600.0;
        let (h_unnormalized, m, s) = split_sexagesimal(ra_seconds, 5);
        let h = h_unnormalized % 24;

        let sign = if self.dec_rad() < 0.0 { '-' } else { '+' };

        let dec_arcseconds = self.dec_deg().abs() * 3600.0;
        let (d, am, arcsec) = split_sexagesimal(dec_arcseconds, 4);

        write!(
            f,
            "{} {h:02}h {m:02}m {s:08.5}s {sign}{d:02}° {am:02}' {arcsec:07.4}\"",
            F::NAME
        )
    }
}
