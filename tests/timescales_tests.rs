use apparent_rs::calendar;
use apparent_rs::timescales;

pub const TOLERANCE: f64 = 1e-9;

#[test]
fn test_epoch_jd() {
    let jd = get_jd(2000, 1, 1, 12, 0, 0.0);
    assert_close(jd.value(), 2451545.0, TOLERANCE)
}

#[test]
fn test_unix_epoch_date() {
    let jd = get_jd(1970, 1, 1, 0, 0, 0.0);
    assert_close(jd.value(), 2440587.5, TOLERANCE)
}

#[test]
fn test_mjd_origin_date() {
    let jd = get_jd(1858, 11, 17, 00, 0, 0.0);
    assert_close(jd.value(), 2400000.5, TOLERANCE)
}

#[test]
fn test_julian_day_zero_date() {
    let jd = get_jd(-4713, 11, 24, 12, 0, 0.0);
    assert_close(jd.value(), 0.0, TOLERANCE)
}

#[test]
fn test_arbitrary_time_of_day() {
    let jd = get_jd(2002, 12, 15, 18, 37, 12.5);
    assert_close(jd.value(), 2452624.27583912037, TOLERANCE)
}

#[test]
fn test_leap_day_jd() {
    let jd = get_jd(2024, 2, 29, 6, 0, 0.0);
    assert_close(jd.value(), 2460369.75, TOLERANCE)
}

#[test]
fn test_non_leap_century_february_end() {
    let jd = get_jd(1900, 2, 28, 23, 59, 59.0);
    assert_close(jd.value(), 2415079.49998842593, TOLERANCE)
}

#[test]
fn test_non_leap_century_march_start() {
    let jd = get_jd(1900, 3, 1, 0, 0, 0.0);
    assert_close(jd.value(), 2415079.5, TOLERANCE)
}

#[test]
fn test_leap_second_jd() {
    let jd = get_jd(2016, 12, 31, 23, 59, 60.0);
    assert_close(jd.value(), 2457754.5, TOLERANCE)
}

#[test]
fn test_sub_second_precision() {
    let jd = get_jd(1999, 12, 31, 23, 59, 59.999);
    assert_close(jd.value(), 2451544.4999999884, TOLERANCE)
}

#[test]
fn test_utc_to_julian_date_roundtrip() {
    let date = calendar::CalendarDate::new(2002, 12, 15, 12, 00, 0.0).expect("Not a valid date!");
    let jd = timescales::calendar_date_to_julian_date(&date);
    let round_trip_date = timescales::julian_date_to_calendar_date(&jd).expect("Not a valid date!");
    assert_eq!(date, round_trip_date);
}

fn get_jd(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> timescales::JdUtc {
    let date = calendar::CalendarDate::new(year, month, day, hour, minute, second)
        .expect("Not a valid date");
    timescales::calendar_date_to_julian_date(&date)
}

#[track_caller]
fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tolerance,
        "expected {expected}, got {actual} (diff {diff}, tolerance {tolerance})"
    );
}
