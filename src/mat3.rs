use crate::vec3::Vec3;
use std::ops::Mul;

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

    pub fn transpose(self) -> Self {
        Self::new(
            Vec3::new(self.row_x.x, self.row_y.x, self.row_z.x),
            Vec3::new(self.row_x.y, self.row_y.y, self.row_z.y),
            Vec3::new(self.row_x.z, self.row_y.z, self.row_z.z),
        )
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.row_x.dot(v), self.row_y.dot(v), self.row_z.dot(v))
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Mat3 {
        let t = other.transpose();

        Mat3::new(t * self.row_x, t * self.row_y, t * self.row_z)
    }
}
