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
    let mut z = v.floor();
    let f = v - z;

    // Get time
    let mut total_seconds = f * 86400.0;
    total_seconds = (total_seconds * 1e6).round() / 1e6;
    if total_seconds == 86400.0 {
        total_seconds = 0.0;
        z += 1.0;
    }

    let whole = total_seconds.floor() as i64;
    let leftover = total_seconds - whole as f64;

    let hour = whole / 3600;
    let minute = (whole % 3600) / 60;
    let second = whole % 60;

    // Century adjustment
    let alpha = ((z - 1867216.25) / 36524.25).floor();
    let a = z + 1.0 + alpha - (alpha / 4.0).floor();
    let b = a + 1524.0;

    // Find Year, Month, and day
    let c = ((b - 122.1_f64) / 365.25).floor();
    let d = (365.25_f64 * c).floor();
    let e = ((b - d) / 30.6001_f64).floor();

    let day = b - d - (30.6001_f64 * e).floor();

    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };

    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

    CalendarDate::new(
        year as i32,
        month as u8,
        day as u8,
        hour as u8,
        minute as u8,
        second as f64 + leftover,
    )
}
