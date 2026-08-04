use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    row_x: Vec3,
    row_y: Vec3,
    row_z: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Self {
        row_x: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        row_y: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        row_z: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
}
