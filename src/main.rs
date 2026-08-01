mod calendar;
mod timescales;

fn main() {
    let date_a = calendar::CalendarDate::new(1992, 12, 15, 12, 0, 0.0).expect("Not a valid date");
    let date_b = calendar::CalendarDate::new(2002, 12, 15, 12, 0, 0.0).expect("Not a valid date");

    println!("{}", date_a >= date_b);

    let latest_correction = timescales::total_correction_since(&date_a);

    println!("{:?}", latest_correction);
}
