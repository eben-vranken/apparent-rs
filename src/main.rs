use apparent_rs::angles;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    let vec_a: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    println!("{:?}", -vec_a * 10.0)
}
