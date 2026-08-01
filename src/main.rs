mod calendar;
mod timescales;

fn main() {
    let epoch_jd = timescales::JdUtc::new(-10000000.0, 0.0);

    let round_trip = timescales::julian_date_to_calendar_date(&epoch_jd);

    println!("{:?}", round_trip)
}
