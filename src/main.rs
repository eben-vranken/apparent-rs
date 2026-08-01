mod calendar;
mod timescales;

fn main() {
    let epoch_date = calendar::CalendarDate::new(2024, 2, 29, 6, 0, 0.0).expect("Not a valid date");
    let epoch_jd = timescales::calendar_date_to_julian_date(&epoch_date);

    let round_trip = timescales::julian_date_to_calendar_date(&epoch_jd);

    println!("{:?}", round_trip)
}
