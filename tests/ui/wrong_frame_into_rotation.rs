use apparent_rs::direction::Direction;
use apparent_rs::frames::{Icrs, MeanOfDate};
use apparent_rs::mat3::Mat3;
use apparent_rs::rotation::Rotation;

fn main() {
    let r: Rotation<MeanOfDate, Icrs> = Rotation::new(Mat3::rz(0.3));
    let d: Direction<Icrs> = Direction::from_spherical(0.9, 0.4);

    let _ = r * d;
}
