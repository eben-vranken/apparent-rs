mod calendar;
mod timescales;

fn main() {
    println!("Hello, stars!");

    println!("{:?}", timescales::JdUtc::now().unwrap());
}
