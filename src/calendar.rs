#[derive(Debug)]
pub struct CalendarDate {
    // Date
    pub year: i32,
    pub month: u8,
    pub day: u8,

    // Time
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
