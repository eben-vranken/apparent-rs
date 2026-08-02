use crate::calendar::{CalendarDate, CalendarError};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum TimeError {
    BeforeUtcEpoch,
}

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

    pub fn day(&self) -> f64 {
        self.day
    }

    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    pub fn now() -> Result<Self, TimeError> {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TimeError::BeforeUtcEpoch)?;

        let secs = since_epoch.as_secs();
        let days = (secs / 86400) as f64;
        let secs_of_day = (secs % 86400) as f64 + f64::from(since_epoch.subsec_nanos()) * 1e-9;

        Ok(Self {
            day: 2440587.5 + days,
            fraction: secs_of_day / 86400.0,
        })
    }

    pub fn from_calendar(date: &CalendarDate) -> Self {
        let (day, fraction) = calendar_to_two_part(date);
        Self { day, fraction }
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

    pub fn new_from_utc(date: &JdUtc) -> Result<Self, TimeError> {
        let delta_at = leap_seconds_at(date)?;
        let offset = (f64::from(delta_at) + 32.184) / 86400.0;

        Ok(Self {
            day: date.day(),
            fraction: date.fraction() + offset,
        })
    }

    pub fn day(&self) -> f64 {
        self.day
    }

    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    pub fn now() -> Result<Self, TimeError> {
        Self::new_from_utc(&JdUtc::now()?)
    }

    pub fn julian_centuries_since_j2000(&self) -> f64 {
        (self.day - 2451545.0 + self.fraction) / 36525.0
    }

    pub fn from_calendar(date: &CalendarDate) -> Self {
        let (day, fraction) = calendar_to_two_part(date);
        Self { day, fraction }
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

    pub fn day(&self) -> f64 {
        self.day
    }

    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    /// DUT1 = UT1 − UTC, seconds.
    /// Source: IERS Bulletin A, https://datacenter.iers.org/data/latestVersion/bulletinA.txt, retrieved 2nd of August 2026.
    pub fn new_from_utc(date: &JdUtc, dut1: f64) -> Self {
        Self {
            day: date.day(),
            fraction: date.fraction() + dut1 / 86400.0,
        }
    }

    pub fn now(dut1: f64) -> Result<Self, TimeError> {
        Ok(Self::new_from_utc(&JdUtc::now()?, dut1))
    }

    pub fn from_calendar(date: &CalendarDate) -> Self {
        let (day, fraction) = calendar_to_two_part(date);
        Self { day, fraction }
    }
}

pub struct LeapSecondEntry {
    pub insert_date: f64,
    pub total_correction: u8,
}

const LEAP_SECONDS_TABLE: &[LeapSecondEntry] = &[
    LeapSecondEntry {
        insert_date: 2441317.5,
        total_correction: 10,
    },
    LeapSecondEntry {
        insert_date: 2441499.5,
        total_correction: 11,
    },
    LeapSecondEntry {
        insert_date: 2441683.5,
        total_correction: 12,
    },
    LeapSecondEntry {
        insert_date: 2442048.5,
        total_correction: 13,
    },
    LeapSecondEntry {
        insert_date: 2442413.5,
        total_correction: 14,
    },
    LeapSecondEntry {
        insert_date: 2442778.5,
        total_correction: 15,
    },
    LeapSecondEntry {
        insert_date: 2443144.5,
        total_correction: 16,
    },
    LeapSecondEntry {
        insert_date: 2443509.5,
        total_correction: 17,
    },
    LeapSecondEntry {
        insert_date: 2443874.5,
        total_correction: 18,
    },
    LeapSecondEntry {
        insert_date: 2444239.5,
        total_correction: 19,
    },
    LeapSecondEntry {
        insert_date: 2444786.5,
        total_correction: 20,
    },
    LeapSecondEntry {
        insert_date: 2445151.5,
        total_correction: 21,
    },
    LeapSecondEntry {
        insert_date: 2445516.5,
        total_correction: 22,
    },
    LeapSecondEntry {
        insert_date: 2446247.5,
        total_correction: 23,
    },
    LeapSecondEntry {
        insert_date: 2447161.5,
        total_correction: 24,
    },
    LeapSecondEntry {
        insert_date: 2447892.5,
        total_correction: 25,
    },
    LeapSecondEntry {
        insert_date: 2448257.5,
        total_correction: 26,
    },
    LeapSecondEntry {
        insert_date: 2448804.5,
        total_correction: 27,
    },
    LeapSecondEntry {
        insert_date: 2449169.5,
        total_correction: 28,
    },
    LeapSecondEntry {
        insert_date: 2449534.5,
        total_correction: 29,
    },
    LeapSecondEntry {
        insert_date: 2450083.5,
        total_correction: 30,
    },
    LeapSecondEntry {
        insert_date: 2450630.5,
        total_correction: 31,
    },
    LeapSecondEntry {
        insert_date: 2451179.5,
        total_correction: 32,
    },
    LeapSecondEntry {
        insert_date: 2453736.5,
        total_correction: 33,
    },
    LeapSecondEntry {
        insert_date: 2454832.5,
        total_correction: 34,
    },
    LeapSecondEntry {
        insert_date: 2456109.5,
        total_correction: 35,
    },
    LeapSecondEntry {
        insert_date: 2457204.5,
        total_correction: 36,
    },
    LeapSecondEntry {
        insert_date: 2457754.5,
        total_correction: 37,
    },
];

pub fn leap_seconds_at(date: &JdUtc) -> Result<u8, TimeError> {
    // Keep guard or index can be below 0
    if date.value() < 2441317.5 {
        return Err(TimeError::BeforeUtcEpoch);
    }

    let jd = date.value();
    let index = LEAP_SECONDS_TABLE.partition_point(|e| e.insert_date <= jd);

    Ok(LEAP_SECONDS_TABLE[index - 1].total_correction)
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

fn calendar_to_two_part(date: &CalendarDate) -> (f64, f64) {
    let hour = f64::from(date.hour());
    let minute = f64::from(date.minute());
    let second = date.second();

    let jdn = calendar_date_to_julian_day_number(date) as f64;

    let day = jdn - 0.5;
    let fraction = (hour * 3600.0 + minute * 60.0 + second) / 86400.0;

    (day, fraction)
}

pub fn julian_date_to_calendar_date(
    day: f64,
    fraction: f64,
) -> Result<CalendarDate, CalendarError> {
    let shifted = day + 0.5;

    // Seperate days and time
    let z0 = shifted.floor();
    let r = shifted - z0;
    let combined = r + fraction;
    let carry = combined.floor();
    let mut z = z0 + carry;
    let f = combined - carry;

    // Get time
    let mut total_seconds = f * 86400.0;
    total_seconds = (total_seconds * 1e9).round() / 1e9;
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
