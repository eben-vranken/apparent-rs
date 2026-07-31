#[derive(Debug)]
pub enum UserError {
    InvalidMonth,
    InvalidHour,
}

#[derive(Debug)]
pub struct CalendarDate {
    // Date
    year: i32,
    month: u8,
    day: u8,

    // Time
    hour: u8,
    minute: u8,
    second: f32,
}

/// I made the calendar date private so I can validate fields
/// i.e. making sure months can't be over 13, hours over 24,...  
impl CalendarDate {
    // Setter
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f32,
    ) -> Result<Self, UserError> {
        if month > 12 || month < 1 {
            return Err(UserError::InvalidMonth);
        }

        if hour > 24 || hour < 0 {
            return Err(UserError::InvalidHour);
        }

        Ok(Self {
            year: year,

            // Will need seperate validation. October 31st works but February 31st does not.
            month: month,

            day: day,

            hour: hour,

            minute: minute.clamp(0, 59),

            second: second.clamp(0.0, 60.0),
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

    pub fn second(&self) -> f32 {
        self.second
    }
}
