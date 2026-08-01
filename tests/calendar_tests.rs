use apparent_rs::calendar::{CalendarDate, UserError};

#[test]
fn test_valid_date() {
    let date = CalendarDate::new(2002, 12, 15, 12, 30, 0.0);
    assert!(date.is_ok());
}

#[test]
fn test_invalid_month_upper_bound() {
    let date = CalendarDate::new(2002, 13, 15, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidMonth))
}

#[test]
fn test_invalid_month_lower_bound() {
    let date = CalendarDate::new(2002, 0, 15, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidMonth))
}

#[test]
fn test_invalid_day_upper_bound() {
    let date = CalendarDate::new(2002, 12, 32, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidDay))
}

#[test]
fn test_invalid_day_lower_bound() {
    let date = CalendarDate::new(2002, 12, 0, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidDay))
}

#[test]
fn test_invalid_leap_year() {
    let date = CalendarDate::new(2002, 02, 29, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidDay))
}

#[test]
fn test_valid_leap_year() {
    let date = CalendarDate::new(2024, 02, 29, 12, 30, 0.0);
    assert!(date.is_ok())
}

#[test]
fn test_in_valid_century_leap_year() {
    let date = CalendarDate::new(1900, 02, 29, 12, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidDay))
}

#[test]
fn test_valid_century_leap_year() {
    let date = CalendarDate::new(2000, 02, 29, 12, 30, 0.0);
    assert!(date.is_ok())
}

#[test]
fn test_all_months_day_limits() {
    let non_leap_year = 2025;
    let month_limits = [
        (1, 31),
        (2, 28),
        (3, 31),
        (4, 30),
        (5, 31),
        (6, 30),
        (7, 31),
        (8, 31),
        (9, 30),
        (10, 31),
        (11, 30),
        (12, 31),
    ];

    for (month, max_day) in month_limits {
        let valid = CalendarDate::new(non_leap_year, month, max_day, 0, 0, 0.0);
        assert!(
            valid.is_ok(),
            "Expected month {} day {} to be valid",
            month,
            max_day
        );

        let invalid = CalendarDate::new(non_leap_year, month, max_day + 1, 0, 0, 0.0);
        assert_eq!(
            invalid,
            Err(UserError::InvalidDay),
            "Expected month {}, day {} to fail",
            month,
            max_day + 1
        )
    }
}

#[test]
fn test_invalid_hour() {
    let date = CalendarDate::new(2002, 12, 15, 24, 30, 0.0);
    assert_eq!(date, Err(UserError::InvalidHour))
}

#[test]
fn test_invalid_minute() {
    let date = CalendarDate::new(2002, 12, 15, 12, 60, 0.0);
    assert_eq!(date, Err(UserError::InvalidMinute))
}

#[test]
fn test_invalid_second() {
    let date = CalendarDate::new(2002, 12, 15, 12, 30, 61.0);
    assert_eq!(date, Err(UserError::InvalidSecond))
}

#[test]
fn test_nan_second_not_accepted() {
    let date = CalendarDate::new(2002, 12, 15, 12, 30, f64::NAN);
    assert_eq!(date, Err(UserError::InvalidSecond))
}

#[test]
fn test_negative_second_not_accepted() {
    let date = CalendarDate::new(2002, 12, 15, 12, 30, -1.0);
    assert_eq!(date, Err(UserError::InvalidSecond))
}

#[test]
fn test_getters() {
    let date = CalendarDate::new(2024, 5, 10, 14, 45, 30.5).unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 5);
    assert_eq!(date.day(), 10);
    assert_eq!(date.hour(), 14);
    assert_eq!(date.minute(), 45);
    assert_eq!(date.second(), 30.5);
}
