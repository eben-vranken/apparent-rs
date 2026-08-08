use apparent_rs::frames::{Icrs, MeanOfDate};
use apparent_rs::mat3::Mat3;
use apparent_rs::rotation::Rotation;

fn main() {
    let r1: Rotation<Icrs, MeanOfDate> = Rotation::new(Mat3::rz(0.3));
    let r2: Rotation<Icrs, MeanOfDate> = Rotation::new(Mat3::rx(0.7));

    let _ = r2 * r1;
}
