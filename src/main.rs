use apparent_rs::constants;

fn main() {
    println!("Hello, stars!");

    println!("{}", constants::TAU / constants::RADIANS_PER_DEGREE);
    println!("{}", constants::TAU / constants::RADIANS_PER_HOUR);
    println!("{}", constants::TAU / constants::RADIANS_PER_ARCSECOND);
    println!("{}", constants::TAU / constants::RADIANS_PER_ARCMILLISECOND);
}
