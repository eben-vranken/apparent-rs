use std::marker::PhantomData;
use std::ops::Mul;

use crate::direction::Direction;
use crate::frames::Frame;
use crate::mat3::Mat3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation<A: Frame, B: Frame> {
    m: Mat3,
    frames: PhantomData<(A, B)>,
}

impl<A: Frame, B: Frame> Rotation<A, B> {
    pub fn new(m: Mat3) -> Self {
        debug_assert!(
            (m.determinant() - 1.0).abs() < 1e-12,
            "rotation matrix must have determinant +1, got {}",
            m.determinant()
        );

        Self {
            m,
            frames: PhantomData,
        }
    }

    pub fn as_mat3(self) -> Mat3 {
        self.m
    }

    pub fn then<C: Frame>(self, next: Rotation<B, C>) -> Rotation<A, C> {
        next * self
    }

    pub fn inverse(self) -> Rotation<B, A> {
        Rotation::new(self.m.transpose())
    }
}

impl<A: Frame, B: Frame> Mul<Direction<A>> for Rotation<A, B> {
    type Output = Direction<B>;

    fn mul(self, d: Direction<A>) -> Direction<B> {
        Direction::new(self.m * d.as_vec3())
    }
}

impl<A: Frame, B: Frame, C: Frame> Mul<Rotation<A, B>> for Rotation<B, C> {
    type Output = Rotation<A, C>;

    fn mul(self, other: Rotation<A, B>) -> Rotation<A, C> {
        Rotation::new(self.m * other.m)
    }
}
