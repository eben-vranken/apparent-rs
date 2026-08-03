use apparent_rs::angles;
use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    let vec_a: Vec3 = Vec3::new(3.0, 4.0, 0.0);

    println!("{:?}", vec_a.normalize().length())
}
