use apparent_rs::constants::RADIANS_PER_DEGREE;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    let vec = Vec3::from_spherical(
        89.99999999 * RADIANS_PER_DEGREE,
        89.99999999 * RADIANS_PER_DEGREE,
    );
    let (lon, lat) = vec.to_spherical();

    println!("{:?}", vec);
    println!("{} {}", lon / RADIANS_PER_DEGREE, lat / RADIANS_PER_DEGREE);
}
