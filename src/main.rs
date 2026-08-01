mod calendar;
mod timescales;

fn main() {
    println!("Hello, stars!");

    let date = calendar::CalendarDate::new(2026, 08, 1, 00, 00, 00.0).expect("Not a valid date");

    let jd = timescales::calendar_date_to_julian_date(&date);

    println!("{:?}", jd.value());
}
