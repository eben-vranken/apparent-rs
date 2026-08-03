use apparent_rs::angles;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    let vec_a: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let vec_b: Vec3 = Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    };

    println!("{:?}", vec_a + vec_b)
}
