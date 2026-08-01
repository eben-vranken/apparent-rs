mod calendar;
mod timescales;

fn main() {
    println!("Hello, stars!");

    let jd = timescales::calendar_date_to_julian_date(
        &calendar::CalendarDate::new(2000, 1, 1, 11, 58, 5.816).unwrap(),
    );

    println!(
        "{:?}",
        timescales::JdTt::new_from_utc(&jd)
            .unwrap()
            .julian_centuries_since_j2000()
    )
}
