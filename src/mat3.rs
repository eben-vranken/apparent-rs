use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    // Similarly to Vec3, I'm okay with these fields being public
    pub row_x: Vec3,
    pub row_y: Vec3,
    pub row_z: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Self {
        row_x: Vec3::X,
        row_y: Vec3::Y,
        row_z: Vec3::Z,
    };

    pub fn new(row_x: Vec3, row_y: Vec3, row_z: Vec3) -> Self {
        Self {
            row_x,
            row_y,
            row_z,
        }
    }
}
