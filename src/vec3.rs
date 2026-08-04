use crate::angles::normalize_2pi;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    // I'm okay with these fields being public, since they don't need any validation
    // Any three numbers are a legitimate field
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    // Predefined constants
    pub const ZERO: Vec3 = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const X: Vec3 = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const Y: Vec3 = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const Z: Vec3 = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();

        self * (1.0 / len)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn from_spherical(lon: f64, lat: f64) -> Vec3 {
        let (lat_sin, lat_cos) = lat.sin_cos();
        let (lon_sin, lon_cos) = lon.sin_cos();

        Self {
            x: lat_cos * lon_cos,
            y: lat_cos * lon_sin,
            z: lat_sin,
        }
    }

    pub fn to_spherical(self) -> (f64, f64) {
        let lon = normalize_2pi(self.y.atan2(self.x));
        let lat = self.z.atan2(self.x.hypot(self.y));

        (lon, lat)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self: Vec3, scale: f64) -> Vec3 {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self: Vec3) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
