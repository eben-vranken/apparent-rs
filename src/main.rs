mod calendar;
mod timescales;

fn main() {
    println!("Hello, stars!");

    println!("{:?}", timescales::JdUtc::now().unwrap());
    println!("{:?}", timescales::JdTt::now().unwrap());
    println!("{:?}", timescales::JdUt1::now(0.9).unwrap());
}
