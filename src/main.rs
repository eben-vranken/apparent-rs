mod calendar;
mod timescales;

fn main() {
    println!("Hello, stars!");

    let jd = timescales::JdTt::from_calendar(
        &calendar::CalendarDate::new(2000, 1, 1, 12, 0, 0.0).unwrap(),
    );

    println!("{:?}", jd.julian_centuries_since_j2000())
}
