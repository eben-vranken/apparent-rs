use std::marker::PhantomData;

use crate::{frames::Frame, vec3::Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction<F: Frame> {
    v: Vec3,
    frame: PhantomData<F>,
}

impl<F: Frame> Direction<F> {
    pub fn new(v: Vec3) -> Self {
        Self {
            v,
            frame: PhantomData,
        }
    }

    pub fn from_spherical(lon: f64, lat: f64) -> Self {
        Self {
            v: Vec3::from_spherical(lon, lat),
            frame: PhantomData,
        }
    }

    pub fn as_vec3(self) -> Vec3 {
        self.v
    }

    pub fn to_spherical(self) -> (f64, f64) {
        self.v.to_spherical()
    }
}
