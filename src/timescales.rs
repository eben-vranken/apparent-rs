use crate::calendar::CalendarDate;

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

pub struct JdTt(pub f64);
pub struct JdUt1(pub f64);

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
