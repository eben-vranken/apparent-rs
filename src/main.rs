mod calendar;
mod timescales;

fn main() {
    let date_a = calendar::CalendarDate::new(2002, 12, 15, 12, 0, 0.0).expect("Not a valid date");
    let date_b = calendar::CalendarDate::new(2002, 12, 15, 13, 0, 0.0).expect("Not a valid date");

    println!("{}", date_a.after(&date_b));

    let epoch_jd = timescales::JdUtc::new(-10000000.0, 0.0);

    let round_trip = timescales::julian_date_to_calendar_date(&epoch_jd);

    println!("{:?}", round_trip)
}
