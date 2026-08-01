#[derive(Debug, PartialEq)]
pub enum CalendarError {
    InvalidMonth,
    InvalidDay,
    InvalidHour,
    InvalidMinute,
    InvalidSecond,
}

/// I made the calendar date private so I can validate fields
/// i.e. making sure months can't be over 13, hours over 24,...  
#[derive(Debug, PartialEq, PartialOrd)]
pub struct CalendarDate {
    // Date
    year: i32,
    month: u8,
    day: u8,

    // Time
    hour: u8,
    minute: u8,
    second: f64,
}

impl CalendarDate {
    const fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    const fn days_in_month(year: i32, month: u8) -> Option<u8> {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
            4 | 6 | 9 | 11 => Some(30),
            2 => {
                if Self::is_leap_year(year) {
                    Some(29)
                } else {
                    Some(28)
                }
            }
            _ => None,
        }
    }

    pub const fn unwrap_const(result: Result<Self, CalendarError>) -> Self {
        match result {
            Ok(date) => date,
            Err(_) => panic!("invalid CalendarDate literal"),
        }
    }

    // Setter
    pub const fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
    ) -> Result<Self, CalendarError> {
        let max_days = match Self::days_in_month(year, month) {
            Some(days) => days,
            None => return Err(CalendarError::InvalidMonth),
        };

        if day < 1 || day > max_days {
            return Err(CalendarError::InvalidDay);
        }

        if hour > 23 {
            return Err(CalendarError::InvalidHour);
        }

        if minute > 59 {
            return Err(CalendarError::InvalidMinute);
        }

        if second.is_nan() || second < 0.0 || second >= 61.0 {
            return Err(CalendarError::InvalidSecond);
        }

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    // Getters
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> f64 {
        self.second
    }
}
