use crate::calendar::{CalendarDate, CalendarError};

#[derive(Debug)]
pub struct JdUtc {
    day: f64,
    fraction: f64,
}

impl JdUtc {
    pub fn new(day: f64, fraction: f64) -> Self {
        Self { day, fraction }
    }

    pub fn value(&self) -> f64 {
        self.day + self.fraction
    }
}

#[derive(Debug)]
pub struct JdTt {
    day: f64,
    fraction: f64,
}

impl JdTt {
    pub fn new(day: f64, fraction: f64) -> Self {
        Self { day, fraction }
    }

    pub fn value(&self) -> f64 {
        self.day + self.fraction
    }
}

#[derive(Debug)]
pub struct JdUt1 {
    day: f64,
    fraction: f64,
}

impl JdUt1 {
    pub fn new(day: f64, fraction: f64) -> Self {
        Self { day, fraction }
    }

    pub fn value(&self) -> f64 {
        self.day + self.fraction
    }
}

pub fn calendar_date_to_julian_day_number(date: &CalendarDate) -> i32 {
    let year = date.year();
    let month = i32::from(date.month());
    let day = i32::from(date.day());

    // Adjust months so Jan = 13, Feb = 14 of the previous year
    let (a_year, a_month) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    day - 32075 + (1461 * (a_year + 4800) / 4) + (367 * (a_month - 2) / 12)
        - (3 * ((a_year + 4900) / 100) / 4)
}

pub fn calendar_date_to_julian_date(date: &CalendarDate) -> JdUtc {
    let hour = f64::from(date.hour());
    let minute = f64::from(date.minute());
    let second = date.second();

    let jdn = calendar_date_to_julian_day_number(date) as f64;

    let day = jdn - 0.5;
    let fraction = (hour * 3600.0 + minute * 60.0 + second) / 86400.0;

    JdUtc::new(day, fraction)
}

pub fn julian_date_to_calendar_date(date: &JdUtc) -> Result<CalendarDate, CalendarError> {
    let v = date.value() + 0.5;

    // Seperate days and time
    let shifted_jd = JdUtc::new(v.trunc(), v.fract());
    let z = shifted_jd.value().trunc();
    let f = shifted_jd.value().fract();

    // Century adjustment
    let mut alpha = 0.0;
    let mut a = 0.0;
    let mut b = 0.0;

    if z >= 2299161.0 {
        alpha = ((z - 1867216.25) / 36524.25).floor();
        a = z + 1.0 + alpha - (alpha / 4.0).floor();
        b = a + 1524.0;
    } else {
        b = z + 1524.0;
    }

    // Find Year, Month, and day
    let c = ((b - 122.1_f64) / 365.25).floor();
    let d = (365.25_f64 * c).floor();
    let e = ((b - d) / 30.6001_f64).floor();

    let day = b - d - (30.6001_f64 * e).floor() + f.floor();

    let mut month = 0.0;

    if e < 14.0 {
        month = e - 1.0;
    } else {
        month = e - 13.0;
    }

    let mut year = 0.0;

    if month > 2.0 {
        year = c - 4716.0;
    } else {
        year = c - 4715.0;
    }

    // Get time
    let mut hour = f * 24.0;

    let mut minute = hour.fract() * 60.0;
    let mut second = minute.fract() * 60.0;

    hour = hour.floor();
    minute = minute.floor();
    second = second.floor();

    CalendarDate::new(
        year as i32,
        month as u8,
        day as u8,
        hour as u8,
        minute as u8,
        second,
    )
}
