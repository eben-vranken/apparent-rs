use apparent_rs::constants::RADIANS_PER_DEGREE;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    println!(
        "{:?}",
        Vec3::from_spherical(279.2348 * RADIANS_PER_DEGREE, 38.78369 * RADIANS_PER_DEGREE)
    );
}
