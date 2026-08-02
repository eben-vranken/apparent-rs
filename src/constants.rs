/// Hard wired constants that never change, per definition
// Time units
pub const SECONDS_PER_MINUTE: f64 = 60.0;
pub const SECONDS_PER_HOUR: f64 = 3_600.0;
pub const SECONDS_PER_DAY: f64 = 86_400.0;
pub const SECONDS_PER_DAY_INT: u64 = 86_400;
pub const NANOS_PER_SECOND: f64 = 1e9;

// Julian dates epochs
pub const J2000_EPOCH_JD: f64 = 2_451_545.0;
pub const UNIX_EPOCH_JD: f64 = 2_440_587.5;
pub const MJD_EPOCH_JD: f64 = 2_400_000.5;

// Julian periods
pub const DAYS_PER_JULIAN_YEAR: f64 = 365.25;
pub const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;
pub const DAYS_PER_JULIAN_MILLENNIUM: f64 = 365_250.0;

// Timescale offsets
pub const TT_MINUS_TAI_SECONDS: f64 = 32.184;

// Calendar Validation
pub const MAX_SECONDS_OF_MINUTE: f64 = 61.0; // Allows for 60.x for leap seconds
