use apparent_rs::vec3::Vec3;

fn main() {
    println!("Hello, stars!");

    let vec_a: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    let vec_b: Vec3 = Vec3::new(1.0, 0.0, 0.0);

    println!("{:?}", vec_a.dot(vec_b))
}
