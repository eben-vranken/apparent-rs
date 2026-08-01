mod calendar;
mod timescales;

fn main() {
    let epoch_date =
        calendar::CalendarDate::new(2000, 01, 01, 12, 00, 00.0).expect("Not a valid date");
    let epoch_jd = timescales::calendar_date_to_julian_date(&epoch_date);
    println!("{:?}", epoch_jd.value());

    let unix_epoch_date =
        calendar::CalendarDate::new(1970, 01, 01, 00, 00, 00.0).expect("Not a valid date");
    let unix_epoch_jd = timescales::calendar_date_to_julian_date(&unix_epoch_date);
    println!("{:?}", unix_epoch_jd.value());

    let mjd_origin_date =
        calendar::CalendarDate::new(1858, 11, 17, 00, 00, 00.0).expect("Not a valid date");
    let mjd_origin_jd = timescales::calendar_date_to_julian_date(&mjd_origin_date);
    println!("{:?}", mjd_origin_jd.value());

    let julian_day_zero_date =
        calendar::CalendarDate::new(-4713, 11, 24, 12, 00, 00.0).expect("Not a valid date");
    let julian_day_zero_jd = timescales::calendar_date_to_julian_date(&julian_day_zero_date);
    println!("{:?}", julian_day_zero_jd.value());
}
